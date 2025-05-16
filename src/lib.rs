pub mod editor;
pub mod syntax;

pub use editor::{
    EditorWidget,
    commands::{EditorMode, VimMode},
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}