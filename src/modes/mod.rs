mod vim;
mod emacs;

pub use vim::VimMode;
pub use emacs::EmacsMode;

use egui::Context;
use crate::editor::{buffer::TextBuffer, commands::EditorCommand};

/// Trait for different editing modes (vim, emacs, etc.)
pub trait EditingMode {
    /// Process keyboard input and return editor commands
    fn process_input(&mut self, ctx: &Context, buffer: &TextBuffer) -> Option<EditorCommand>;
    
    /// Return the name of this editing mode
    fn name(&self) -> &'static str;
    
    /// Returns true if the editor is in insert mode (text can be directly modified)
    fn is_insert_mode(&self) -> bool;
}