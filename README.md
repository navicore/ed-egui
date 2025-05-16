# Ed-Egui

A flexible code editor widget for egui with Vim and Emacs key bindings.

## Features

- First-class support for both Vim and Emacs key bindings
- Native integration with egui
- Compatible with bevy_egui
- Syntax highlighting for multiple languages
- Mixed content support (markdown with embedded code blocks)
- Line numbering
- Extensible architecture

## Usage

### Basic Usage

```rust
use ed_egui::{EditorWidget, EditingMode};

// Create an editor with default settings (Emacs mode)
let mut editor = EditorWidget::default();

// Show the editor in an egui UI
ui.add(&mut editor);
```

### With Vim Keybindings

```rust
use ed_egui::{EditorWidget, EditingMode};

// Create an editor with Vim mode
let mut editor = EditorWidget::default()
    .with_mode(ed_egui::modes::EditingModeType::Vim);

// Show the editor in an egui UI
ui.add(&mut editor);
```

### With Syntax Highlighting

```rust
use ed_egui::{EditorWidget, EditingMode};
use ed_egui::syntax::MarkdownHighlighter;

// Create an editor with syntax highlighting
let mut editor = EditorWidget::default()
    .with_syntax_highlighter(MarkdownHighlighter::new());

// Show the editor in an egui UI
ui.add(&mut editor);
```

## Examples

The crate comes with several examples:

1. `simple_editor` - A basic editor with both Vim and Emacs modes
2. `markdown_editor` - A markdown editor with syntax highlighting and preview

Run them with:

```bash
cargo run --example simple_editor --features eframe-demo
cargo run --example markdown_editor --features eframe-demo
```

## Vim Mode Features

The Vim mode implementation supports:

- Modal editing (Normal, Insert, Visual)
- Movement commands (h, j, k, l, w, b, 0, $)
- Operators (d, c, y) with motions
- Counts (e.g., 5j to move down 5 lines)
- Text objects (coming soon)

## Emacs Mode Features

The Emacs mode implementation supports:

- Chord-based key bindings (C-x C-s, etc.)
- Navigation commands (C-f, C-b, C-p, C-n, C-a, C-e)
- Mark and region operations
- Kill and yank (cut and paste)

## Project Status

This project is in early development. Contributions and feedback are welcome!

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license

at your option.

## Bevy Integration

This crate works with bevy_egui out of the box. Simply use the `bevy` feature to enable bevy_egui support:

```rust
ed-egui = { version = "0.1", features = ["bevy"] }
```

Then use the editor widget in your bevy_egui UI code:

```rust
fn ui_system(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        let mut editor = EditorWidget::default();
        editor.show(ui);
    });
}
```