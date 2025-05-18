//!! This file is part of the `rustpad` project, which is licensed under the Apache License 2.0.
pub mod editor;
pub mod syntax;

// Re-export the main components for easier access
pub use editor::{
    commands::{EditorMode, VimMode},
    EditorWidget,
};
