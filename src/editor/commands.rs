/// Types of cursor movement supported by the editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMovement {
    Left,
    Right,
    Up,
    Down,
    WordLeft,
    WordRight,
    LineStart,
    LineEnd,
    DocumentStart,
    DocumentEnd,
}

/// Represents commands that can be executed on the text buffer
#[derive(Debug, Clone)]
pub enum EditorCommand {
    // Character operations
    InsertChar(char),
    DeleteChar,
    DeleteCharForward,

    // Cursor movement
    MoveCursor(CursorMovement),

    // Text operations
    DeleteLine,
    DeleteWord,
    Copy,
    Cut,
    Paste,
    NewLine,

    // Custom commands
    Custom(String),

    // Mode switching
    ChangeMode(EditorMode),
}

/// Editor mode (Vim or Emacs)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Vim(VimMode),
    Emacs,
}

impl Default for EditorMode {
    fn default() -> Self {
        Self::Emacs
    }
}

/// Vim editor modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
}

impl Default for VimMode {
    fn default() -> Self {
        Self::Normal
    }
}
