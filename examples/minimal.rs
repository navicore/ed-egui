//! Minimal example showing basic integration of the ed-egui widget
//!
//! This example demonstrates the absolute minimum code needed to use the editor
//! without focusing on any specific keybinding mode. For examples that focus on
//! a specific editing mode, see:
//! - vim_editor.rs - For Vim keybindings
//! - emacs_editor.rs - For Emacs keybindings

use ed_egui::EditorWidget;
use eframe::egui;

struct MinimalEditorApp {
    editor: EditorWidget,
}

impl Default for MinimalEditorApp {
    fn default() -> Self {
        // Create a new editor with some basic settings
        let mut editor = EditorWidget::new("minimal")
            .with_font_size(14.0)
            .with_status_bar(true);

        // Set initial text content
        editor.set_text("Hello, ed-egui!\n\nThis is the minimal example.\n\nBy default, this uses Emacs keybindings.\nTo use Vim mode, see the vim_editor.rs example.");

        Self { editor }
    }
}

impl eframe::App for MinimalEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Minimal Editor Example");
            ui.label("The simplest possible integration of ed-egui with an egui app.");

            ui.separator();

            // Just show the editor - that's it!
            self.editor.show(ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Minimal Ed-Egui Example",
        native_options,
        Box::new(|_cc| Box::new(MinimalEditorApp::default())),
    )
}
