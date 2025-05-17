pub mod buffer;
pub mod commands;

use egui::{Color32, Context, Event, Response, RichText, TextEdit, Ui};

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
    /// Track the last inserted character position for VIM normal mode
    last_cursor_pos: usize,
    /// Store keyboard events that shouldn't be processed by TextEdit
    suppressed_events: Vec<Event>,
}

impl EditorWidget {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            buffer: TextBuffer::default(),
            current_mode: EditorMode::Emacs, // Default to Emacs mode
            font_size: 14.0,
            show_status: true,
            last_cursor_pos: 0,
            suppressed_events: Vec::new(),
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

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.current_mode = mode;
    }

    /// The key method for the editor widget - this function:
    /// 1. Intercepts keyboard events that would normally go to the TextEdit
    /// 2. Processes vim commands directly on the buffer
    /// 3. Prevents unwanted characters from being inserted in normal mode
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        // 1. Capture the current cursor position before any edits
        self.last_cursor_pos = self.buffer.cursor_position();
        
        // 2. Process key events BEFORE we create the TextEdit widget
        self.process_input_before_ui(ui.ctx());
        
        // 3. Show mode indicator at the top of the editor
        match self.current_mode {
            EditorMode::Vim(VimMode::Normal) => {
                ui.label(RichText::new("-- VIM: NORMAL MODE --").strong().monospace().color(Color32::GREEN));
            }
            EditorMode::Vim(VimMode::Insert) => {
                ui.label(RichText::new("-- VIM: INSERT MODE --").strong().monospace().color(Color32::YELLOW));
            }
            EditorMode::Emacs => {
                ui.label(RichText::new("-- EMACS MODE --").strong().monospace().color(Color32::LIGHT_BLUE));
            }
            _ => {}
        }
        
        // 4. Create a layouter for basic syntax highlighting
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
        
        // 5. Create the basic text edit widget
        let mut text_edit = TextEdit::multiline(self.buffer.text_mut())
            .id_source(format!("{}_edit", self.id))
            .font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .layouter(&mut layouter);
            
        // 6. Add more styling/configuration to the text edit
        match self.current_mode {
            EditorMode::Vim(VimMode::Normal) => {
                text_edit = text_edit.hint_text("Normal mode: press 'i' to edit");
            }
            EditorMode::Vim(VimMode::Insert) => {
                text_edit = text_edit.hint_text("Insert mode: press Escape to exit");
            }
            EditorMode::Emacs => {
                text_edit = text_edit.hint_text("Emacs mode");
            }
            _ => {}
        }
        
        // 7. Add the text edit to the UI
        let mut response = ui.add(text_edit);
        
        // 8. We need to restore cursor position in VIM normal mode
        // because the TextEdit widget might have changed it due to keyboard events
        if matches!(self.current_mode, EditorMode::Vim(VimMode::Normal)) {
            if self.buffer.cursor_position() != self.last_cursor_pos {
                self.buffer.set_cursor_position(self.last_cursor_pos);
            }
        }

        // 9. Show status bar if enabled
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
        
        // 10. If focus is lost but we're in VIM normal mode, try to keep focus
        if !response.has_focus() && matches!(self.current_mode, EditorMode::Vim(VimMode::Normal)) {
            response.request_focus();
        }

        response
    }
    
    /// Intercept and process keyboard input before the UI is created
    fn process_input_before_ui(&mut self, ctx: &Context) {
        // We need to manipulate the input events to prevent unwanted character insertion
        ctx.input_mut(|input| {
            // Track events we want to suppress (to be removed from input events)
            let mut events_to_remove = Vec::new();
            
            // First check for mode transitions
            let is_vim_normal = matches!(self.current_mode, EditorMode::Vim(VimMode::Normal));
            let is_vim_insert = matches!(self.current_mode, EditorMode::Vim(VimMode::Insert));
            
            // Process keyboard events (individual keys)
            for key in &input.keys_down {
                // Handle Escape to exit insert mode
                if *key == egui::Key::Escape && is_vim_insert {
                    self.current_mode = EditorMode::Vim(VimMode::Normal);
                    // Mark all events for removal to avoid unwanted text modifications
                    events_to_remove.extend(0..input.events.len());
                    // We don't want to process further events
                    break;
                }
                
                // Handle normal mode commands
                if is_vim_normal {
                    match *key {
                        // Mode transitions
                        egui::Key::I if input.key_pressed(egui::Key::I) => {
                            self.current_mode = EditorMode::Vim(VimMode::Insert);
                            // Mark all events for removal to avoid the 'i' being inserted
                            events_to_remove.extend(0..input.events.len());
                            break;
                        },
                        
                        // Movement
                        egui::Key::H if input.key_pressed(egui::Key::H) => {
                            self.execute_command(EditorCommand::MoveCursor(CursorMovement::Left));
                            events_to_remove.extend(0..input.events.len());
                        },
                        egui::Key::J if input.key_pressed(egui::Key::J) => {
                            self.execute_command(EditorCommand::MoveCursor(CursorMovement::Down));
                            events_to_remove.extend(0..input.events.len());
                        },
                        egui::Key::K if input.key_pressed(egui::Key::K) => {
                            self.execute_command(EditorCommand::MoveCursor(CursorMovement::Up));
                            events_to_remove.extend(0..input.events.len());
                        },
                        egui::Key::L if input.key_pressed(egui::Key::L) => {
                            self.execute_command(EditorCommand::MoveCursor(CursorMovement::Right));
                            events_to_remove.extend(0..input.events.len());
                        },
                        
                        // Word movement
                        egui::Key::W if input.key_pressed(egui::Key::W) => {
                            self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordRight));
                            events_to_remove.extend(0..input.events.len());
                        },
                        egui::Key::B if input.key_pressed(egui::Key::B) => {
                            self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordLeft));
                            events_to_remove.extend(0..input.events.len());
                        },
                        
                        // Line movement
                        egui::Key::Num0 if input.key_pressed(egui::Key::Num0) => {
                            self.execute_command(EditorCommand::MoveCursor(CursorMovement::LineStart));
                            events_to_remove.extend(0..input.events.len());
                        },
                        egui::Key::End if input.key_pressed(egui::Key::End) => {
                            self.execute_command(EditorCommand::MoveCursor(CursorMovement::LineEnd));
                            events_to_remove.extend(0..input.events.len());
                        },
                        
                        // Document movement
                        egui::Key::G if input.key_pressed(egui::Key::G) => {
                            if input.modifiers.shift {
                                self.execute_command(EditorCommand::MoveCursor(CursorMovement::DocumentEnd));
                            } else {
                                self.execute_command(EditorCommand::MoveCursor(CursorMovement::DocumentStart));
                            }
                            events_to_remove.extend(0..input.events.len());
                        },
                        
                        // Editing
                        egui::Key::X if input.key_pressed(egui::Key::X) => {
                            self.execute_command(EditorCommand::DeleteCharForward);
                            events_to_remove.extend(0..input.events.len());
                        },
                        
                        _ => {}
                    }
                }
            }
            
            // Now handle Text events for normal mode - suppress ALL of them
            if is_vim_normal {
                // In normal mode, find all Text events and mark them for removal
                for (i, event) in input.events.iter().enumerate() {
                    match event {
                        Event::Text(_) => {
                            if !events_to_remove.contains(&i) {
                                events_to_remove.push(i);
                            }
                        },
                        _ => {}
                    }
                }
            }
            
            // Remove events in reverse order to maintain correct indices
            events_to_remove.sort_unstable();
            events_to_remove.dedup();
            for &index in events_to_remove.iter().rev() {
                if index < input.events.len() {
                    input.events.remove(index);
                }
            }
            
            // Handle Emacs key commands
            if matches!(self.current_mode, EditorMode::Emacs) {
                // Process CTRL key combinations for Emacs mode
                if input.modifiers.ctrl {
                    // Basic movement
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
                    
                    // Line movement
                    if input.key_pressed(egui::Key::A) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::LineStart));
                    }
                    if input.key_pressed(egui::Key::E) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::LineEnd));
                    }
                }
                
                // Process ALT key combinations for Emacs mode
                if input.modifiers.alt {
                    // Word movement
                    if input.key_pressed(egui::Key::F) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordRight));
                    }
                    if input.key_pressed(egui::Key::B) {
                        self.execute_command(EditorCommand::MoveCursor(CursorMovement::WordLeft));
                    }
                }
            }
        });
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
            _ => {} // Other commands not yet implemented
        }
    }
}