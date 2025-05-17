pub mod buffer;
pub mod commands;
pub mod emacs_handler;
pub mod keyhandler;
pub mod vim_handler;

use egui::{Color32, Context, Event, Key, Response, RichText, TextEdit, Ui};

use crate::syntax::HighlightOptions;

use self::buffer::TextBuffer as BufferImpl;
use self::commands::{EditorMode, VimMode};
use self::emacs_handler::EmacsKeyHandler;
use self::keyhandler::KeyHandler;
use self::vim_handler::VimKeyHandler;

/// The main editor widget that implements a simple code editor
pub struct EditorWidget {
    /// The unique ID for the editor instance
    id: String,
    /// The text buffer that holds the content of the editor
    buffer: BufferImpl,
    /// The current mode of the editor (Vim or Emacs)
    current_mode: EditorMode,
    /// The font size for the editor
    font_size: f32,
    /// Whether to show the status bar at the bottom
    show_status: bool,
    /// Track the last inserted character position for VIM normal mode
    last_cursor_pos: usize,
    /// Vim key handler for vim mode
    vim_handler: VimKeyHandler,
    /// Emacs key handler for emacs mode
    emacs_handler: EmacsKeyHandler,
}

impl Default for EditorWidget {
    fn default() -> Self {
        Self {
            id: String::new(),
            buffer: BufferImpl::default(),
            current_mode: EditorMode::Emacs, // Default to Emacs mode
            font_size: 14.0,
            show_status: true,
            last_cursor_pos: 0,
            vim_handler: VimKeyHandler::new().with_debug(true),
            emacs_handler: EmacsKeyHandler::new().with_debug(true),
        }
    }
}

impl EditorWidget {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            buffer: BufferImpl::default(),
            current_mode: EditorMode::Emacs, // Default to Emacs mode
            font_size: 14.0,
            show_status: true,
            last_cursor_pos: 0,
            vim_handler: VimKeyHandler::new().with_debug(true),
            emacs_handler: EmacsKeyHandler::new().with_debug(true),
        }
    }

    #[must_use]
    pub const fn with_mode(mut self, mode: EditorMode) -> Self {
        self.current_mode = mode;
        self
    }

    #[must_use]
    pub const fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    #[must_use]
    pub const fn with_status_bar(mut self, show: bool) -> Self {
        self.show_status = show;
        self
    }

    pub fn text(&self) -> &str {
        self.buffer.text()
    }

    pub const fn text_mut(&mut self) -> &mut String {
        self.buffer.text_mut()
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.buffer.set_text(text.into());
    }

    pub const fn mode(&self) -> &EditorMode {
        &self.current_mode
    }

    pub const fn set_mode(&mut self, mode: EditorMode) {
        self.current_mode = mode;

        // Update the vim handler mode if needed
        if let EditorMode::Vim(vim_mode) = mode {
            self.vim_handler.set_mode(vim_mode);
        }
    }

    /// The key method for the editor widget - this function:
    /// 1. Intercepts keyboard events that would normally go to the `TextEdit`
    /// 2. Processes vim commands directly on the buffer
    /// 3. Prevents unwanted characters from being inserted in normal mode
    #[allow(clippy::too_many_lines)]
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        // 1. Process key events BEFORE we create the TextEdit widget
        self.process_input_before_ui(ui.ctx());

        // 2. Show mode indicator at the top of the editor
        match self.current_mode {
            EditorMode::Vim(VimMode::Normal) => {
                ui.label(
                    RichText::new("-- VIM: NORMAL MODE --")
                        .strong()
                        .monospace()
                        .color(Color32::GREEN),
                );
            }
            EditorMode::Vim(VimMode::Insert) => {
                ui.label(
                    RichText::new("-- VIM: INSERT MODE --")
                        .strong()
                        .monospace()
                        .color(Color32::YELLOW),
                );
            }
            EditorMode::Vim(VimMode::Visual) => {
                ui.label(
                    RichText::new("-- VIM: VISUAL MODE --")
                        .strong()
                        .monospace()
                        .color(Color32::GOLD),
                );
            }
            EditorMode::Emacs => {
                ui.label(
                    RichText::new("-- EMACS MODE --")
                        .strong()
                        .monospace()
                        .color(Color32::LIGHT_BLUE),
                );
            }
        }

        // 3. Create a layouter for basic syntax highlighting
        let font_size = self.font_size;
        let mut layouter = move |ui: &Ui, text: &str, _wrap_width: f32| {
            let mut options = HighlightOptions {
                font_size: 14.0,
                text_color: Color32::from_rgb(220, 223, 228),
                keyword_color: Color32::from_rgb(198, 120, 221),
                comment_color: Color32::from_rgb(92, 99, 112),
                heading_color: Color32::from_rgb(229, 192, 123),
            };
            options.font_size = font_size;

            let layout_job = crate::syntax::basic_highlight(text, &options);
            ui.fonts(|fonts| fonts.layout_job(layout_job))
        };

        // 4. Create a TextEdit widget for all modes - unified approach
        // Create the TextEdit widget
        let id = format!("{}_edit", self.id);
        let mut text_edit = TextEdit::multiline(self.buffer.text_mut())
            .id_source(id)
            .font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .layouter(&mut layouter);

        // Add styling based on mode
        text_edit = match self.current_mode {
            EditorMode::Vim(VimMode::Normal) => {
                text_edit.hint_text("Normal mode: press 'i' to edit, 'v' for visual mode")
            }
            EditorMode::Vim(VimMode::Insert) => {
                text_edit.hint_text("Insert mode: press Escape to exit")
            }
            EditorMode::Vim(VimMode::Visual) => {
                text_edit.hint_text("Visual mode: use movement keys to select, 'y' to copy, 'x/d' to cut, 'c' to change, 'p' to replace")
            }
            EditorMode::Emacs => text_edit.hint_text("Emacs mode"),
        };

        // 5. Add the text edit to the UI and get the output
        // We now use .show instead of ui.add to get access to cursor_range
        let output = text_edit.show(ui);
        let response = output.response;

        // 6. Sync our buffer's cursor position with TextEdit's cursor position
        if let Some(cursor_range) = output.cursor_range {
            // Get the character range from the cursor
            let range = cursor_range.as_sorted_char_range();
            // Use the end position of the selection as our cursor
            let cursor_pos = range.end;
            // Update our buffer's cursor position to match TextEdit's
            self.buffer.set_cursor_position(cursor_pos);
            println!("DEBUG: Updated buffer cursor position to {}", cursor_pos);
        }

        // 7. In vim normal or visual mode, ensure that the editor retains focus
        if matches!(
            self.current_mode,
            EditorMode::Vim(VimMode::Normal | VimMode::Visual)
        ) && !response.has_focus()
        {
            response.request_focus();
        }

        // 7. Show status bar if enabled
        if self.show_status {
            ui.horizontal(|ui| {
                // Show current mode
                let (mode_text, mode_color) = match self.current_mode {
                    EditorMode::Vim(VimMode::Normal) => ("VIM: NORMAL", Color32::GREEN),
                    EditorMode::Vim(VimMode::Insert) => ("VIM: INSERT", Color32::YELLOW),
                    EditorMode::Vim(VimMode::Visual) => ("VIM: VISUAL", Color32::GOLD),
                    EditorMode::Emacs => ("EMACS", Color32::LIGHT_BLUE),
                };

                ui.label(
                    RichText::new(mode_text)
                        .monospace()
                        .strong()
                        .color(mode_color),
                );

                // Show cursor position
                let cursor_pos = self.buffer.cursor_position();
                let line = self.buffer.current_line();
                let column = self.buffer.current_column();
                ui.label(
                    RichText::new(format!(
                        "Pos: {} (L:{}, C:{})",
                        cursor_pos,
                        line + 1,
                        column + 1
                    ))
                    .monospace(),
                );

                // Add a spacer to push the right-side content
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("Chars: {}", self.buffer.text().len())).monospace(),
                    );
                });
            });
        }

        response
    }

    /// Intercept and process keyboard input before the UI is created
    fn process_input_before_ui(&mut self, ctx: &Context) {
        
        // We need to manipulate the input events to handle our custom key bindings
        ctx.input_mut(|input| {
            // Enhanced debug print of all input events
            if !input.events.is_empty() {
                println!("Input events BEFORE processing: {:?}", input.events);
            }

            // Debug print of input keys with more detail
            if !input.keys_down.is_empty() {
                println!(
                    "Keys down BEFORE processing: {:?}, modifiers: {:?}",
                    input.keys_down, input.modifiers
                );

                // This area was used for debugging problematic keys
                // Now we have a robust solution that handles both Key and Text events

                // Check specifically for Ctrl+Arrow key combinations
                if input.modifiers.ctrl {
                    for key in &input.keys_down {
                        if matches!(key, Key::ArrowLeft | Key::ArrowRight) {
                            println!("DEBUG: Detected Ctrl+Arrow key: {:?}", key);

                            if input.key_pressed(*key) {
                                println!("DEBUG: Ctrl+Arrow key was just pressed");

                                // Let TextEdit handle Ctrl+Arrow keys for word movement
                                println!("DEBUG: Letting TextEdit handle Ctrl+Arrow keys natively");
                            }
                        }
                    }
                }
            }

            // Events we want to remove
            let mut events_to_remove;

            // Process events based on current mode
            match self.current_mode {
                EditorMode::Vim(_) => {
                    // Use the dedicated Vim key handler
                    events_to_remove = self.vim_handler.process_input(ctx, input);
                    println!(
                        "DEBUG: After vim_handler.process_input - events to remove: {:?}",
                        events_to_remove
                    );

                    // Look for custom cursor movement signals
                    let mut custom_events_to_remove = Vec::new();
                    for (i, event) in input.events.iter().enumerate() {
                        if let Event::Text(text) = event {
                            if text.starts_with("__CUSTOM_CURSOR__:") {
                                custom_events_to_remove.push(i);
                            }
                        }
                    }
                    events_to_remove.extend(custom_events_to_remove);

                    // Update last cursor position for Vim normal mode after commands
                    if matches!(self.current_mode, EditorMode::Vim(VimMode::Normal)) {
                        self.last_cursor_pos = self.buffer.cursor_position();
                    }

                    // Sync the editor mode with the handler
                    self.current_mode = EditorMode::Vim(self.vim_handler.mode());
                }
                EditorMode::Emacs => {
                    // Use the dedicated Emacs key handler
                    // The Emacs handler now generates TextEdit-compatible events directly
                    events_to_remove = self.emacs_handler.process_input(ctx, input);

                    // No need to process commands here anymore since the emacs_handler
                    // is now directly generating TextEdit events just like vim_handler
                    println!(
                        "DEBUG: Emacs handler processed input - events to remove: {:?}",
                        events_to_remove
                    );
                }
            }

            // Debug print after processing
            println!("Input events AFTER processing: {:?}", input.events);

            // Remove events in reverse order to maintain correct indices
            events_to_remove.sort_unstable();
            events_to_remove.dedup();

            for &index in events_to_remove.iter().rev() {
                if index < input.events.len() {
                    input.events.remove(index);
                }
            }
        });
    }
    

}
