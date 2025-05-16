use super::EditingMode;
use crate::editor::{
    buffer::TextBuffer,
    commands::{CursorMovement, EditorCommand, VimMode as VimModeEnum},
};
use egui::{Context, Key};

/// Implementation of Vim-style modal editing
pub struct VimMode {
    mode: VimModeEnum,
    command_buffer: String,
    count: Option<usize>,
    operator: Option<VimOperator>,
    pending_motion: bool,
    register: Option<char>,
}

/// Vim editing operators like delete, change, yank
#[derive(Debug, Clone, Copy)]
enum VimOperator {
    Delete,
    Change,
    Yank,
    Indent,
    Outdent,
}

impl Default for VimMode {
    fn default() -> Self {
        Self {
            mode: VimModeEnum::Normal,
            command_buffer: String::new(),
            count: None,
            operator: None,
            pending_motion: false,
            register: None,
        }
    }
}

impl VimMode {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the current vim mode
    pub fn mode(&self) -> VimModeEnum {
        self.mode
    }

    /// Set the vim mode
    pub fn set_mode(&mut self, mode: VimModeEnum) {
        self.mode = mode;
        // Clear command state when changing modes
        self.command_buffer.clear();
        self.count = None;
        self.operator = None;
        self.pending_motion = false;
    }

    /// Process a count digit (0-9)
    fn process_count(&mut self, digit: char) -> Option<EditorCommand> {
        if !digit.is_ascii_digit() {
            return None;
        }

        if digit == '0' && self.count.is_none() {
            // '0' as a command goes to beginning of line
            return Some(EditorCommand::MoveCursor(CursorMovement::LineStart));
        }

        let digit_val = digit.to_digit(10).unwrap() as usize;
        self.count = Some(self.count.unwrap_or(0) * 10 + digit_val);
        None
    }

    /// Process normal mode keys
    fn process_normal_mode(&mut self, key: Key) -> Option<EditorCommand> {
        // First check for count digits
        if let Key::Num0
        | Key::Num1
        | Key::Num2
        | Key::Num3
        | Key::Num4
        | Key::Num5
        | Key::Num6
        | Key::Num7
        | Key::Num8
        | Key::Num9 = key
        {
            let digit = match key {
                Key::Num0 => '0',
                Key::Num1 => '1',
                Key::Num2 => '2',
                Key::Num3 => '3',
                Key::Num4 => '4',
                Key::Num5 => '5',
                Key::Num6 => '6',
                Key::Num7 => '7',
                Key::Num8 => '8',
                Key::Num9 => '9',
                _ => unreachable!(),
            };

            return self.process_count(digit);
        }

        // Then handle operators that expect a motion
        if self.pending_motion {
            return self.process_pending_motion(key);
        }

        // Otherwise process keys normally
        match key {
            // Mode switches
            Key::I => {
                self.set_mode(VimModeEnum::Insert);
                Some(EditorCommand::ChangeMode(VimModeEnum::Insert))
            }
            Key::A => {
                self.set_mode(VimModeEnum::Insert);
                Some(EditorCommand::MoveCursor(CursorMovement::Right))
            }

            // Basic movement
            Key::H => Some(EditorCommand::MoveCursor(CursorMovement::Left)),
            Key::J => Some(EditorCommand::MoveCursor(CursorMovement::Down)),
            Key::K => Some(EditorCommand::MoveCursor(CursorMovement::Up)),
            Key::L => Some(EditorCommand::MoveCursor(CursorMovement::Right)),

            // Word movement
            Key::W => Some(EditorCommand::MoveCursor(CursorMovement::WordRight)),
            Key::B => Some(EditorCommand::MoveCursor(CursorMovement::WordLeft)),

            // Line movement
            Key::Dollar => Some(EditorCommand::MoveCursor(CursorMovement::LineEnd)),
            Key::Caret => Some(EditorCommand::MoveCursor(CursorMovement::LineStart)),

            // Delete operations
            Key::X => Some(EditorCommand::DeleteCharForward),

            // Operators
            Key::D => {
                self.operator = Some(VimOperator::Delete);
                self.pending_motion = true;
                None
            }

            // Other commands
            _ => None,
        }
    }

    /// Process keys when an operator is pending and needs a motion
    fn process_pending_motion(&mut self, key: Key) -> Option<EditorCommand> {
        let operator = self.operator.unwrap();
        self.pending_motion = false;

        match key {
            // If the same key as operator is pressed again, operate on the whole line
            Key::D if operator == VimOperator::Delete => {
                self.operator = None;
                Some(EditorCommand::DeleteLine)
            }

            // Motion commands
            Key::H => self.apply_operator_with_motion(CursorMovement::Left),
            Key::L => self.apply_operator_with_motion(CursorMovement::Right),
            Key::W => self.apply_operator_with_motion(CursorMovement::WordRight),

            // If no valid motion, cancel the operation
            _ => {
                self.operator = None;
                None
            }
        }
    }

    /// Apply the current operator with the given motion
    fn apply_operator_with_motion(&mut self, motion: CursorMovement) -> Option<EditorCommand> {
        let operator = self.operator.take()?;

        match operator {
            VimOperator::Delete => {
                // TODO: Implement multi-step operations
                // For now, just return basic commands
                match motion {
                    CursorMovement::WordRight => Some(EditorCommand::Custom("delete_word".into())),
                    _ => Some(EditorCommand::DeleteChar),
                }
            }
            _ => None, // Other operators not implemented yet
        }
    }
}

impl EditingMode for VimMode {
    fn process_input(&mut self, ctx: &Context, buffer: &TextBuffer) -> Option<EditorCommand> {
        let input = ctx.input();

        // Check for control/escape keys first
        if input.key_pressed(Key::Escape) {
            self.set_mode(VimModeEnum::Normal);
            return Some(EditorCommand::ChangeMode(VimModeEnum::Normal));
        }

        match self.mode {
            VimModeEnum::Normal => {
                // Look for pressed keys
                for key in &input.keys_down {
                    if input.key_pressed(*key) {
                        return self.process_normal_mode(*key);
                    }
                }
            }
            VimModeEnum::Insert => {
                // In insert mode, handle character input directly
                for event in &input.events {
                    if let egui::Event::Text(text) = event {
                        // Just handle the first character for simplicity
                        if let Some(c) = text.chars().next() {
                            return Some(EditorCommand::InsertChar(c));
                        }
                    }
                }

                // Handle special keys in insert mode
                if input.key_pressed(Key::Backspace) {
                    return Some(EditorCommand::DeleteChar);
                }

                if input.key_pressed(Key::Enter) {
                    return Some(EditorCommand::NewLine);
                }
            }
            _ => {
                // Other modes not implemented yet
            }
        }

        None
    }

    fn name(&self) -> &'static str {
        "vim"
    }

    fn is_insert_mode(&self) -> bool {
        matches!(self.mode, VimModeEnum::Insert)
    }
}
