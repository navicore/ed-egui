pub mod editor;
pub mod syntax;
#[cfg(test)]
mod tests;

pub use editor::{
    commands::{EditorMode, VimMode},
    EditorWidget,
};
