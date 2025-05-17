[![Rust CI](https://github.com/navicore/ed-egui/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/navicore/ed-egui/actions/workflows/rust-ci.yml)

# Ed-Egui

A flexible code editor widget for egui with Vim and Emacs key bindings.

# UNDER CONSTRUCTION

# UNDER CONSTRUCTION

# UNDER CONSTRUCTION

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
use ed_egui::EditorWidget;

// Create an editor with default settings (Emacs mode)
let mut editor = EditorWidget::default();

// Show the editor in an egui UI
editor.show(ui);
```

### With Vim Keybindings

```rust
use ed_egui::{EditorWidget, EditorMode, VimMode};

// Create an editor with Vim mode
let mut editor = EditorWidget::default()
    .with_mode(EditorMode::Vim(VimMode::Normal));

// Show the editor in an egui UI
editor.show(ui);
```

### With Font Size and Status Bar

```rust
use ed_egui::EditorWidget;

// Create an editor with custom font size and status bar
let mut editor = EditorWidget::new("my_editor")
    .with_font_size(16.0)
    .with_status_bar(true);

// Set initial content
editor.set_text("Hello, ed-egui!");

// Show the editor in an egui UI
editor.show(ui);
```

## Examples

The crate comes with focused examples to demonstrate different usage scenarios:

1. `minimal` - A minimal example showing basic editor integration
2. `vim_editor` - A dedicated editor using Vim keybindings only
3. `emacs_editor` - A dedicated editor using Emacs keybindings only

Run them with:

```bash
cargo run --example minimal
cargo run --example vim_editor
cargo run --example emacs_editor
```

## Vim Mode Features

The Vim mode implementation supports:

- Modal editing (Normal, Insert, Visual)
- Movement commands (h, j, k, l, w, b, 0, $, g, G)
- Visual mode selections with v
- Visual mode operations (y for copy, d/x for cut, c for change)
- Delete with x in normal mode

## Emacs Mode Features

The Emacs mode implementation supports:

- Standard Emacs navigation commands
  - Ctrl+F/B - Move cursor right/left
  - Ctrl+P/N - Move cursor up/down
  - Ctrl+A/E - Move to start/end of line
  - Alt+F/B - Word movement
  - Alt+< / Alt+> - Document start/end
- Works alongside standard system keyboard shortcuts for editing

## Project Status

This project is in early development. Contributions and feedback are welcome!

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license

at your option.

## Bevy Integration

This crate works with bevy_egui out of the box. Simply add both dependencies to your Bevy project:

```toml
[dependencies]
bevy = "0.12"
bevy_egui = "0.23"
ed-egui = "0.1"
```

Then use the editor widget in your bevy_egui UI code:

```rust
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use ed_egui::{EditorWidget, EditorMode, VimMode};

fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("Editor").show(contexts.ctx_mut(), |ui| {
        let mut editor = EditorWidget::new("bevy_editor")
            .with_mode(EditorMode::Vim(VimMode::Normal))
            .with_status_bar(true);
            
        editor.show(ui);
    });
}
```
