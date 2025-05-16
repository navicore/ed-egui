pub mod buffer;
pub mod commands;

use egui::{Color32, Context, Response, RichText, TextEdit, Ui};

use crate::syntax::HighlightOptions;

use self::buffer::TextBuffer;
use self::commands::{CursorMovement, EditorCommand, EditorMode, VimMode};

/// The main editor widget that implements a simple code editor
#[derive(Default)]
pub struct EditorWidget {
    /// The unique ID for the editor instance
    id: String,
    /// The text buffer that holds the content of the editor
    buffer: TextBuffer,
    /// The current mode of the editor (Vim or Emacs)
    current_mode: EditorMode,
    /// The font size for the editor
    font_size: f32,
    /// Whether to show the status bar at the bottom
    show_status: bool,
}

impl EditorWidget {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            buffer: TextBuffer::default(),
            current_mode: EditorMode::Emacs, // Default to Emacs mode
            font_size: 14.0,
            show_status: true,
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
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        // Create a layouter for basic syntax highlighting
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

        // Create the text edit widget
        let text_edit = TextEdit::multiline(self.buffer.text_mut())
            .id_source(format!("{}_edit", self.id))
            .font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .layouter(&mut layouter);

        let response = ui.add(text_edit);

        // Show status bar if enabled
        if self.show_status {
            ui.horizontal(|ui| {
                // Show current mode
                let mode_text = match self.current_mode {
                    EditorMode::Vim(VimMode::Normal) => "VIM: NORMAL",
                    EditorMode::Vim(VimMode::Insert) => "VIM: INSERT",
                    EditorMode::Vim(VimMode::Visual) => "VIM: VISUAL",
                    EditorMode::Emacs => "EMACS",
                };

                ui.label(RichText::new(mode_text).monospace().strong());

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

        // Process keyboard input if we have focus
        if response.has_focus() {
            self.process_input(ui.ctx());
        }

        response
    }

    /// Process input events for the editor
    fn process_input(&mut self, ctx: &Context) {
        fn fun_name(i: &egui::InputState) -> egui::InputState {
            i.clone()
        }
        let input = ctx.input(fun_name);

        // Check for mode switches
        if input.key_pressed(egui::Key::Escape)
            && self.current_mode == EditorMode::Vim(VimMode::Insert)
        {
            self.current_mode = EditorMode::Vim(VimMode::Normal);
        }

        if self.current_mode == EditorMode::Vim(VimMode::Normal) && input.key_pressed(egui::Key::I)
        {
            self.current_mode = EditorMode::Vim(VimMode::Insert);
        }

        // Process keystrokes based on mode
        match self.current_mode {
            EditorMode::Vim(VimMode::Normal) => {
                // Basic Vim normal mode commands
                // Basic movement
                if input.key_pressed(egui::Key::H) {
                    self.execute_command(EditorCommand::MoveCursor(CursorMovement::Left));
                }
                if input.key_pressed(egui::Key::L) {
                    self.execute_command(EditorCommand::MoveCursor(CursorMovement::Right));
                }
                if input.key_pressed(egui::Key::J) {
                    self.execute_command(EditorCommand::MoveCursor(CursorMovement::Down));
                }
                if input.key_pressed(egui::Key::K) {
                    self.execute_command(EditorCommand::MoveCursor(CursorMovement::Up));
                }

                // Word movement
                if input.key_pressed(egui::Key::W) {
                    self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordRight));
                }
                if input.key_pressed(egui::Key::B) {
                    self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordLeft));
                }

                // Line movement
                if input.key_pressed(egui::Key::Num0) {
                    self.execute_command(EditorCommand::MoveCursor(CursorMovement::LineStart));
                }
                if input.key_pressed(egui::Key::End)
                    || (input.key_pressed(egui::Key::Semicolon) && input.modifiers.shift)
                {
                    self.execute_command(EditorCommand::MoveCursor(CursorMovement::LineEnd));
                }

                // Document movement
                if input.key_pressed(egui::Key::G) {
                    if input.modifiers.shift {
                        self.execute_command(EditorCommand::MoveCursor(
                            CursorMovement::DocumentEnd,
                        ));
                    } else {
                        self.execute_command(EditorCommand::MoveCursor(
                            CursorMovement::DocumentStart,
                        ));
                    }
                }

                // Editing
                if input.key_pressed(egui::Key::X) {
                    self.execute_command(EditorCommand::DeleteCharForward);
                }
            }
            EditorMode::Vim(VimMode::Insert) | EditorMode::Emacs => {
                // Handle text input
                for event in &input.events {
                    if let egui::Event::Text(text) = event {
                        for c in text.chars() {
                            self.execute_command(EditorCommand::InsertChar(c));
                        }
                    }
                }

                // Handle special keys
                if input.key_pressed(egui::Key::Backspace) {
                    self.execute_command(EditorCommand::DeleteChar);
                }
                if input.key_pressed(egui::Key::Enter) {
                    self.execute_command(EditorCommand::NewLine);
                }

                // Emacs-style movement
                if input.modifiers.ctrl {
                    if input.key_pressed(egui::Key::F) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::Right));
                    }
                    if input.key_pressed(egui::Key::B) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::Left));
                    }
                    if input.key_pressed(egui::Key::P) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::Up));
                    }
                    if input.key_pressed(egui::Key::N) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::Down));
                    }
                    if input.key_pressed(egui::Key::A) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::LineStart));
                    }
                    if input.key_pressed(egui::Key::E) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::LineEnd));
                    }
                }

                // Alt-based word movement in Emacs
                if input.modifiers.alt {
                    if input.key_pressed(egui::Key::F) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordRight));
                    }
                    if input.key_pressed(egui::Key::B) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordLeft));
                    }
                    if input.key_pressed(egui::Key::ArrowRight) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordRight));
                    }
                    if input.key_pressed(egui::Key::ArrowLeft) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordLeft));
                    }
                }

                // Document movement
                if input.modifiers.alt {
                    if input.key_pressed(egui::Key::Period) && input.modifiers.shift {
                        // M->
                        self.execute_command(EditorCommand::MoveCursor(
                            CursorMovement::DocumentEnd,
                        ));
                    }
                    if input.key_pressed(egui::Key::Comma) && input.modifiers.shift {
                        // M-<
                        self.execute_command(EditorCommand::MoveCursor(
                            CursorMovement::DocumentStart,
                        ));
                    }
                }
            }
            EditorMode::Vim(_) => {}
        }
    }

    /// Execute an editor command
    fn execute_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::InsertChar(c) => self.buffer.insert_char(c),
            EditorCommand::DeleteChar => self.buffer.delete_char(),
            EditorCommand::DeleteCharForward => self.buffer.delete_char_forward(),
            EditorCommand::MoveCursor(movement) => match movement {
                CursorMovement::Left => self.buffer.move_cursor_left(),
                CursorMovement::Right => self.buffer.move_cursor_right(),
                CursorMovement::Up => self.buffer.move_cursor_up(),
                CursorMovement::Down => self.buffer.move_cursor_down(),
                CursorMovement::LineStart => self.buffer.move_to_line_start(),
                CursorMovement::LineEnd => self.buffer.move_to_line_end(),
                CursorMovement::WordLeft => self.buffer.move_cursor_word_left(),
                CursorMovement::WordRight => self.buffer.move_cursor_word_right(),
                CursorMovement::DocumentStart => self.buffer.move_cursor_document_start(),
                CursorMovement::DocumentEnd => self.buffer.move_cursor_document_end(),
            },
            EditorCommand::NewLine => self.buffer.insert_newline(),
            EditorCommand::ChangeMode(mode) => self.current_mode = mode,
            // Other commands not yet implemented
            _ => {}
        }
    }
}
