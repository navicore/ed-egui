use egui::{Context, Key, Modifiers};
use crate::editor::{
    buffer::TextBuffer,
    commands::{CursorMovement, EditorCommand},
};
use super::EditingMode;

/// Implementation of Emacs-style key bindings
pub struct EmacsMode {
    // For multi-key sequences like C-x C-s
    pending_prefix: Option<EmacsPrefix>,
    mark_active: bool,
}

/// Emacs command prefixes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EmacsPrefix {
    CtrlX,
    CtrlC,
    Alt,
}

impl Default for EmacsMode {
    fn default() -> Self {
        Self {
            pending_prefix: None,
            mark_active: false,
        }
    }
}

impl EmacsMode {
    pub fn new() -> Self {
        Self::default()
    }
    
    fn process_ctrl_key(&mut self, key: Key, modifiers: Modifiers) -> Option<EditorCommand> {
        match key {
            // Basic movement
            Key::B => Some(EditorCommand::MoveCursor(CursorMovement::Left)),
            Key::F => Some(EditorCommand::MoveCursor(CursorMovement::Right)),
            Key::P => Some(EditorCommand::MoveCursor(CursorMovement::Up)),
            Key::N => Some(EditorCommand::MoveCursor(CursorMovement::Down)),
            
            // Line movement
            Key::A => Some(EditorCommand::MoveCursor(CursorMovement::LineStart)),
            Key::E => Some(EditorCommand::MoveCursor(CursorMovement::LineEnd)),
            
            // Word movement (with Alt modifier)
            Key::ArrowLeft if modifiers.alt => Some(EditorCommand::MoveCursor(CursorMovement::WordLeft)),
            Key::ArrowRight if modifiers.alt => Some(EditorCommand::MoveCursor(CursorMovement::WordRight)),
            
            // Delete operations
            Key::D => Some(EditorCommand::DeleteCharForward),
            Key::H => Some(EditorCommand::DeleteChar), // Backspace
            
            // Buffer operations
            Key::X => {
                self.pending_prefix = Some(EmacsPrefix::CtrlX);
                None
            },
            
            // Mark operations
            Key::Space => {
                self.mark_active = true;
                Some(EditorCommand::Custom("set_mark".into()))
            },
            
            // Other commands
            _ => None,
        }
    }
    
    fn process_ctrl_x_prefix(&mut self, key: Key, modifiers: Modifiers) -> Option<EditorCommand> {
        self.pending_prefix = None;
        
        match key {
            // Save (C-x C-s)
            Key::S if modifiers.ctrl => Some(EditorCommand::Custom("save_buffer".into())),
            
            // Kill region (C-x C-k)
            Key::K if modifiers.ctrl => {
                if self.mark_active {
                    self.mark_active = false;
                    Some(EditorCommand::Cut)
                } else {
                    None
                }
            },
            
            // Clipboard operations
            Key::C => Some(EditorCommand::Copy),
            Key::V => Some(EditorCommand::Paste),
            
            _ => None,
        }
    }
}

impl EditingMode for EmacsMode {
    fn process_input(&mut self, ctx: &Context, buffer: &TextBuffer) -> Option<EditorCommand> {
        let input = ctx.input();
        
        // Handle pending prefix if any
        if let Some(prefix) = self.pending_prefix {
            for key in &input.keys_down {
                if input.key_pressed(*key) {
                    match prefix {
                        EmacsPrefix::CtrlX => return self.process_ctrl_x_prefix(*key, input.modifiers),
                        _ => {
                            // Other prefixes not implemented yet
                            self.pending_prefix = None;
                        }
                    }
                }
            }
        }
        
        // Check for control key combinations
        if input.modifiers.ctrl {
            for key in &input.keys_down {
                if input.key_pressed(*key) {
                    return self.process_ctrl_key(*key, input.modifiers);
                }
            }
        }
        
        // Handle normal character input
        for event in &input.events {
            if let egui::Event::Text(text) = event {
                // Just handle the first character for simplicity
                if let Some(c) = text.chars().next() {
                    return Some(EditorCommand::InsertChar(c));
                }
            }
        }
        
        // Handle special keys
        if input.key_pressed(Key::Backspace) {
            return Some(EditorCommand::DeleteChar);
        }
        
        if input.key_pressed(Key::Enter) {
            return Some(EditorCommand::NewLine);
        }
        
        None
    }
    
    fn name(&self) -> &'static str {
        "emacs"
    }
    
    fn is_insert_mode(&self) -> bool {
        // Emacs is always in "insert mode" - text can always be edited directly
        true
    }
}