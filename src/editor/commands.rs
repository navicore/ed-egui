/// Represents commands that can be executed on the text buffer
#[derive(Debug, Clone)]
pub enum EditorCommand {
    // Character operations
    InsertChar(char),
    DeleteChar,
    DeleteCharForward,

    // Cursor movement
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
    MoveToLineStart,
    MoveToLineEnd,

    // Line operations
    NewLine,

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
