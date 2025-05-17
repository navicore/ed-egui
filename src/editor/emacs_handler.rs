use crate::editor::commands::{CursorMovement, EditorCommand};
use crate::editor::keyhandler::KeyHandler;
use egui::{Context, InputState, Key};

/// Implements Emacs key handling for the editor

#[derive(Default)]
pub struct EmacsKeyHandler {
    /// Debug printing enabled/disabled
    debug: bool,
    /// Commands that need to be executed
    pub commands: Vec<EditorCommand>,
}

impl EmacsKeyHandler {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Enable or disable debug logging
    fn debug_log(&self, message: &str) {
        if self.debug {
            println!("[EmacsKeyHandler] {message}");
        }
    }

    /// Clear any commands that have been queued up
    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }
}

impl KeyHandler for EmacsKeyHandler {
    fn process_input(&mut self, _ctx: &Context, input: &mut InputState) -> Vec<usize> {
        // Clear any previous commands
        self.commands.clear();

        // Emacs uses control and alt key combinations for most commands
        // These don't interfere with normal typing, so we don't need to remove events

        // Process CTRL key combinations
        if input.modifiers.ctrl {
            // Basic movement
            if input.key_pressed(Key::F) {
                self.debug_log("Ctrl+F pressed - cursor right");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::Right));
            }
            if input.key_pressed(Key::B) {
                self.debug_log("Ctrl+B pressed - cursor left");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::Left));
            }
            if input.key_pressed(Key::P) {
                self.debug_log("Ctrl+P pressed - cursor up");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::Up));
            }
            if input.key_pressed(Key::N) {
                self.debug_log("Ctrl+N pressed - cursor down");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::Down));
            }

            // Line movement
            if input.key_pressed(Key::A) {
                self.debug_log("Ctrl+A pressed - line start");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::LineStart));
            }
            if input.key_pressed(Key::E) {
                self.debug_log("Ctrl+E pressed - line end");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::LineEnd));
            }

            // Document movement
            if input.key_pressed(Key::Home) {
                self.debug_log("Ctrl+Home pressed - document start");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::DocumentStart));
            }
            if input.key_pressed(Key::End) {
                self.debug_log("Ctrl+End pressed - document end");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::DocumentEnd));
            }
        }

        // Process ALT (Meta) key combinations
        if input.modifiers.alt {
            // Word movement
            if input.key_pressed(Key::F) {
                self.debug_log("Alt+F pressed - word right");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::WordRight));
            }
            if input.key_pressed(Key::B) {
                self.debug_log("Alt+B pressed - word left");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::WordLeft));
            }

            // Document movement
            if input.key_pressed(Key::Comma) && input.modifiers.shift {
                self.debug_log("Alt+< pressed - document start");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::DocumentStart));
            }
            if input.key_pressed(Key::Period) && input.modifiers.shift {
                self.debug_log("Alt+> pressed - document end");
                self.commands
                    .push(EditorCommand::MoveCursor(CursorMovement::DocumentEnd));
            }
        }

        // Emacs mode generally doesn't need to remove events, since it works with key modifiers
        Vec::new()
    }

    fn name(&self) -> &'static str {
        "emacs"
    }
}
