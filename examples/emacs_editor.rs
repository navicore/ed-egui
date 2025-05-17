//! Emacs-specific editor example
//!
//! This example demonstrates how to create an editor that exclusively 
//! uses Emacs keybindings. It shows how to:
//! - Initialize the editor in Emacs mode
//! - Provide a cheat sheet of available Emacs commands
//! - Integrate with an egui application

use ed_egui::{EditorMode, EditorWidget};
use eframe::egui;

struct EmacsEditorApp {
    editor: EditorWidget,
}

impl Default for EmacsEditorApp {
    fn default() -> Self {
        // Create a new editor specifically in Emacs mode
        let mut editor = EditorWidget::new("emacs_editor")
            .with_mode(EditorMode::Emacs) // Set to Emacs mode
            .with_font_size(16.0)
            .with_status_bar(true);

        // Set initial sample text with Emacs commands cheat sheet
        editor.set_text(
            r#"# Emacs Mode Editor

This is a specialized editor that only uses Emacs keybindings.

## Available Commands

### Cursor Movement
- Basic: Use `Ctrl+F/B/P/N` for right/left/up/down
- By Word: Use `Alt+F/B` to move forward/backward by word
- By Line: Use `Ctrl+A/E` to move to start/end of line
- By Document: 
  - Use `Alt+<` (Alt+Shift+,) for document start
  - Use `Alt+>` (Alt+Shift+.) for document end
  - Or use `Ctrl+Home/End` for document start/end

### Editing
- Standard text typing works as expected
- Use platform standard shortcuts for copy/paste/etc.

Try it out!
"#,
        );

        Self { editor }
    }
}

impl eframe::App for EmacsEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Emacs Mode Editor");
                
                // Display mode info
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("EMACS MODE");
                });
            });

            ui.separator();

            // Show the editor with emacs bindings
            self.editor.show(ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ed-Egui Emacs Editor",
        native_options,
        Box::new(|_cc| Box::new(EmacsEditorApp::default())),
    )
}