# Ed-Egui Architecture

## Project Overview

Ed-Egui aims to create a flexible code editor widget for egui with first-class support for both Vim and Emacs key bindings. The widget should be usable within any egui application, including bevy_egui.

## Design Goals

1. **Native-Feeling Key Bindings**: Implement complete, accurate key bindings for both Vim and Emacs
2. **No Surprises for Power Users**: Maintain expected behavior for users familiar with these editors
3. **Modular Architecture**: Support both editing paradigms without compromising either
4. **Easy Integration**: Drop-in widget for any egui application
5. **Extensibility**: Allow for custom key bindings and editing modes
6. **Performance**: Maintain egui's immediate mode model while handling complex editing operations
7. **Mixed Content Support**: Handle markdown with embedded code blocks in multiple languages

## Core Components

### 1. EditorWidget

The main widget that integrates with egui. Responsible for:
- Rendering text with syntax highlighting
- Managing scroll views and layout
- Processing input and delegating to the appropriate editing mode
- Exposing a simple API for applications

### 2. EditingCore

The core text editing functionality, independent of key bindings:
- Text buffer management
- Cursor and selection handling
- Undo/redo history
- Text operations (insert, delete, replace)

### 3. EditingMode (Trait)

Interface for different editing paradigms:
- Process keyboard input
- Maintain mode state (Normal, Insert, Visual for Vim; regular for Emacs)
- Convert key sequences to editing commands

### 4. KeyBindings

Two primary implementations:
- **VimBindings**: Modal editing with command sequences
- **EmacsBindings**: Chord-based editing with extensive use of modifiers

### 5. ContentType Management

System to detect and track content types within a document:
- Markdown parser to identify code blocks and their languages
- Content region tracking (prose vs. code)
- Language-specific behaviors (indentation, commenting)

### 6. Multi-Language Syntax Highlighting

Enhanced highlighting system:
- Token-based lexing with context awareness
- Support for switching languages within a document based on markdown fences (```java, ```rust, etc.)
- Theme support with consistent styling across languages
- Language definitions for common programming languages
- Support for showing LLM assistant output with mixed content

## Input Handling Flow

1. Keyboard input events come from egui
2. EditorWidget checks if it has focus
3. If focused, input is passed to the current EditingMode
4. EditingMode processes key sequences/chords according to its paradigm
5. EditingMode generates editor commands
6. EditCore executes commands on the text buffer
7. UI is updated with new buffer state

## Implementation Challenges

### 1. Input Handling Strategy

Egui has limited keyboard event handling capabilities. We need to:
- Track key states ourselves for complex key sequences
- Handle modifiers consistently across platforms
- Work around egui limitations without forking it

### 2. Vim Mode Implementation

- Maintain modal state (Normal, Insert, Visual, etc.)
- Support command composition (`dw`, `ci"`, etc.)
- Implement operator-pending mode
- Handle counts and registers

### 3. Emacs Key Chord Support

- Support complex modifier combinations (Ctrl+X Ctrl+S, etc.)
- Implement mark/region system
- Support minibuffer-like functionality

### 4. Mixed Content Handling

- Parse markdown to identify code blocks and their languages
- Switch syntax highlighting based on content type
- Support special editing behaviors in different regions (code vs. prose)
- Handle proper indentation in code blocks
- Update highlighting when fence language identifiers change

### 5. Platform Considerations

- Handle key event differences between web and native
- Ensure consistent behavior across operating systems
- Account for different keyboard layouts

## Integration with egui

- Use the existing TextEdit widget as a foundation
- Override key handling for specialized behavior
- Implement a custom layouter for syntax highlighting
- Provide configuration options matching existing egui patterns

## Integration with bevy_egui

Since bevy_egui is just a wrapper around egui, our editor should work without any special accommodations.

## Mixed Content Implementation Strategy

For handling mixed markdown and code content:

1. **Incremental Parsing**:
   - Parse document to identify content regions
   - Cache region information for performance
   - Update incrementally when content changes

2. **Language Detection**:
   - Parse markdown code fence info strings (```java, ```python)
   - Map to appropriate syntax highlighter
   - Support default fallback for unknown languages

3. **Context-Aware Editing**:
   - Adjust key binding behavior based on content type (e.g., different tab behavior)
   - Provide language-specific indentation and commenting
   - Special handling for editing fence boundaries

4. **Mixed Content Rendering**:
   - Use different text styles for markdown vs. code
   - Support nested highlighting (e.g., inline code within markdown)
   - Maintain consistent visual themes across content types

This approach will support displaying LLM assistant output where explanatory text and code examples are intermixed, providing appropriate styling and editing behavior for each content type.

## Future Extensions

- Language server protocol (LSP) integration
- Plugin system for custom commands
- Additional editing paradigms (VSCode, Sublime, etc.)
- Vertical selection and multiple cursors
- Real-time collaboration
- Folding for code blocks and markdown sections