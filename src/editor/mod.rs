pub mod buffer;
pub mod commands;

use egui::{Response, RichText, TextEdit, Ui, Context};

use self::buffer::TextBuffer;
use self::commands::{EditorMode, VimMode};

/// The main editor widget that implements a simple code editor
#[derive(Default)]
pub struct EditorWidget {
    id: String,
    buffer: TextBuffer,
    current_mode: EditorMode,
    font_size: f32,
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

    pub fn with_mode(mut self, mode: EditorMode) -> Self {
        self.current_mode = mode;
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_status_bar(mut self, show: bool) -> Self {
        self.show_status = show;
        self
    }

    pub fn text(&self) -> &str {
        self.buffer.text()
    }

    pub fn text_mut(&mut self) -> &mut String {
        self.buffer.text_mut()
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.buffer.set_text(text.into());
    }

    pub fn mode(&self) -> &EditorMode {
        &self.current_mode
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.current_mode = mode;
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        // Create a layouter for basic syntax highlighting
        let font_size = self.font_size;
        let mut layouter = move |ui: &Ui, text: &str, _wrap_width: f32| {
            let mut options = crate::syntax::HighlightOptions::default();
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
                ui.label(RichText::new(format!("Cursor: {}", cursor_pos)).monospace());
                
                // Add a spacer to push the right-side content
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("Chars: {}", self.buffer.text().len())).monospace());
                });
            });
        }
        
        // Process keyboard input if we have focus
        if response.has_focus() {
            self.process_input(ui.ctx());
        }
        
        response
    }
    
    fn process_input(&mut self, ctx: &Context) {
        let input = ctx.input(|i| i.clone());
        
        // Check for mode switches
        if input.key_pressed(egui::Key::Escape) {
            if let EditorMode::Vim(VimMode::Insert) = self.current_mode {
                self.current_mode = EditorMode::Vim(VimMode::Normal);
            }
        }
        
        if let EditorMode::Vim(VimMode::Normal) = self.current_mode {
            if input.key_pressed(egui::Key::I) {
                self.current_mode = EditorMode::Vim(VimMode::Insert);
            }
        }
        
        // Process keystrokes based on mode
        match self.current_mode {
            EditorMode::Vim(VimMode::Normal) => {
                // Basic Vim normal mode commands
                if input.key_pressed(egui::Key::H) {
                    self.buffer.move_cursor_left();
                }
                if input.key_pressed(egui::Key::L) {
                    self.buffer.move_cursor_right();
                }
                if input.key_pressed(egui::Key::X) {
                    self.buffer.delete_char_forward();
                }
                if input.key_pressed(egui::Key::End) {
                    self.buffer.move_to_line_end();
                }
                if input.key_pressed(egui::Key::Num0) {
                    self.buffer.move_to_line_start();
                }
            },
            EditorMode::Vim(VimMode::Insert) | EditorMode::Emacs => {
                // Handle text input
                for event in &input.events {
                    if let egui::Event::Text(text) = event {
                        for c in text.chars() {
                            self.buffer.insert_char(c);
                        }
                    }
                }
                
                // Handle special keys
                if input.key_pressed(egui::Key::Backspace) {
                    self.buffer.delete_char();
                }
                if input.key_pressed(egui::Key::Enter) {
                    self.buffer.insert_newline();
                }
                
                // Emacs-style movement
                if input.modifiers.ctrl {
                    if input.key_pressed(egui::Key::F) {
                        self.buffer.move_cursor_right();
                    }
                    if input.key_pressed(egui::Key::B) {
                        self.buffer.move_cursor_left();
                    }
                    if input.key_pressed(egui::Key::A) {
                        self.buffer.move_to_line_start();
                    }
                    if input.key_pressed(egui::Key::E) {
                        self.buffer.move_to_line_end();
                    }
                }
            },
            _ => {}
        }
    }
}