use ed_egui::{EditorMode, EditorWidget, VimMode};
use eframe::egui;

struct BasicEditorApp {
    editor: EditorWidget,
}

impl Default for BasicEditorApp {
    fn default() -> Self {
        let mut editor = EditorWidget::default()
            .with_font_size(16.0)
            .with_status_bar(true);

        // Set some initial text
        editor.set_text("# Basic Editor Demo\n\nThis is a simple editor that supports Vim and Emacs keybindings.\n\n## Vim Commands\n\n- Press `Esc` to enter Normal mode\n- Press `i` to enter Insert mode\n- In Normal mode, use `h/l` to move cursor left/right\n- Use `0/$` to move to start/end of line\n- Use `x` to delete character under cursor\n\n## Emacs Commands\n\n- Use `Ctrl+F/B` to move cursor right/left\n- Use `Ctrl+A/E` to move to start/end of line\n");

        Self { editor }
    }
}

impl eframe::App for BasicEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Ed-Egui Basic Editor");

                ui.separator();

                // Switch between editing modes
                if ui.button("Vim Mode").clicked() {
                    self.editor.set_mode(EditorMode::Vim(VimMode::Normal));
                }

                if ui.button("Emacs Mode").clicked() {
                    self.editor.set_mode(EditorMode::Emacs);
                }
            });

            ui.separator();

            // Show the editor
            self.editor.show(ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ed-Egui Basic Editor",
        native_options,
        Box::new(|_cc| Box::new(BasicEditorApp::default())),
    )
}
