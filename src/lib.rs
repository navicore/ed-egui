//!! This file is part of the `rustpad` project, which is licensed under the Apache License 2.0.
pub mod editor;
pub mod syntax;
#[cfg(test)]
mod tests;

pub use editor::{
    commands::{EditorMode, VimMode},
    EditorWidget,
};
