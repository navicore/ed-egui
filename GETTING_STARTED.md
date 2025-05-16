# Getting Started with Ed-Egui

This document provides basic instructions for building and running the editor prototype.

## Prerequisites

You'll need:
- Rust and Cargo installed (https://rustup.rs/)
- A C/C++ compiler toolchain
- Git

## Building the Project

1. Clone the repository:
   ```
   git clone https://github.com/navicore/ed-egui.git
   cd ed-egui
   ```

2. Build the project:
   ```
   cargo build
   ```

## Running the Examples

The project includes several examples to demonstrate different aspects of the editor:

### Basic Editor

This example demonstrates basic editing functionality with both Vim and Emacs keybindings:

```
cargo run --example basic_editor --features eframe-demo
```

### Minimal Demo

This is a simplified demo showing egui's built-in text editing capabilities:

```
cargo run --example minimal --features eframe-demo
```

## Using the Editor

The editor supports two main editing modes:

### Vim Mode

When in Vim mode:
- Press `Esc` to switch to Normal mode
- Press `i` to enter Insert mode
- In Normal mode:
  - Use `h` and `l` to move the cursor left and right
  - Use `0` to move to the start of the line
  - Use `$` to move to the end of the line
  - Use `x` to delete the character under the cursor

### Emacs Mode

In Emacs mode:
- Type normally to insert text
- Use `Ctrl+F` and `Ctrl+B` to move the cursor right and left
- Use `Ctrl+A` to move to the start of the line
- Use `Ctrl+E` to move to the end of the line

## Project Structure

The project has a modular structure:

- `src/editor/` - Core editing functionality
- `src/syntax/` - Syntax highlighting
- `examples/` - Example applications

## Next Steps

After getting comfortable with the basic functionality, you can:

1. Improve the key binding implementation with more commands
2. Enhance the syntax highlighting for multiple languages
3. Add support for more advanced editing features
4. Create a bevy_egui integration example

Refer to the `ARCHITECTURE.md` file for a detailed overview of the project's design and future plans.