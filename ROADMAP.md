# Ed-Egui Development Roadmap

This document outlines the planned development path for the Ed-Egui project.

## Phase 1: Prototype (Current)

- [x] Basic text buffer implementation
- [x] Simple editor widget with egui integration
- [x] Mode switching between Vim and Emacs
- [x] Basic key handling for essential commands
- [x] Minimal syntax highlighting
- [x] Status bar with mode information

## Phase 2: Core Functionality

- [ ] Improved text buffer with efficient line handling
- [ ] Proper cursor navigation (by line, word, character)
- [ ] Selection handling (char, word, line, block)
- [ ] Undo/redo system
- [ ] Copy/cut/paste operations
- [ ] Line number display
- [ ] Scrolling improvements

## Phase 3: Advanced Editing

### Vim Mode Enhancements
- [ ] Complete normal mode commands
- [ ] Visual mode selections (char, line, block)
- [ ] Command-line mode with basic commands
- [ ] Text objects (word, sentence, paragraph)
- [ ] Operators (delete, change, yank)
- [ ] Marks and registers
- [ ] Search and replace
- [ ] Macros

### Emacs Mode Enhancements
- [ ] Complete keybinding set
- [ ] Kill ring and yank
- [ ] Region operations
- [ ] Rectangle editing
- [ ] Multiple cursors
- [ ] Incremental search
- [ ] Keyboard macros

## Phase 4: Enhanced Syntax and Languages

- [ ] Multi-language syntax highlighting
- [ ] Markdown with embedded code blocks
- [ ] Context-aware indentation
- [ ] Auto-pairing of brackets, quotes, etc.
- [ ] Code folding
- [ ] Language-specific features

## Phase 5: Performance and Advanced Features

- [ ] Efficient handling of large files
- [ ] Asynchronous operations
- [ ] Language Server Protocol (LSP) integration
- [ ] Theming and customization
- [ ] Plugin system
- [ ] Configuration file support

## Phase 6: Platform Integration

- [ ] Bevy integration showcase application
- [ ] Support for different platforms (desktop, web)
- [ ] Feature parity across platforms
- [ ] Published crate with documented API

## Milestones

### 0.1.0 - Prototype Release
- Basic editor with minimal Vim and Emacs keybindings
- Simple text editing and navigation
- Basic status display

### 0.2.0 - Essential Editing
- Complete cursor movement
- Selection support
- Copy/paste operations
- Undo/redo
- Line numbers

### 0.3.0 - Modal Editing
- Full Vim normal mode commands
- Visual selections
- Emacs region commands
- Search functionality

### 0.4.0 - Syntax and Language
- Multi-language support
- Markdown with code blocks
- Code indentation
- Auto-pairing

### 0.5.0 - Advanced Features
- Performance optimizations
- LSP integration
- Theming
- Configuration

### 1.0.0 - Production Release
- Complete feature set
- Comprehensive documentation
- Multiple integration examples
- High test coverage