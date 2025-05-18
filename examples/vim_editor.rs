//! Vim-specific editor example
//!
//! This example demonstrates how to create an editor that exclusively
//! uses Vim keybindings. It shows how to:
//! - Initialize the editor in Vim mode
//! - Display the current Vim mode (normal, insert, visual)
//! - Provide a cheat sheet of available Vim commands

use ed_egui::{EditorMode, EditorWidget, VimMode};
use eframe::egui;

struct VimEditorApp {
    editor: EditorWidget,
}

impl Default for VimEditorApp {
    fn default() -> Self {
        // Create a new editor specifically in Vim mode
        let mut editor = EditorWidget::new("vim_editor")
            .with_mode(EditorMode::Vim(VimMode::Normal)) // Start in Normal mode
            .with_font_size(16.0)
            .with_status_bar(true);

        // Set initial sample text with Vim commands cheat sheet
        editor.set_text(
            r#"# Vim Mode Editor

This is a specialized editor that only uses Vim keybindings.

## Available Commands

### Mode Switching
- Press `i` to enter Insert mode
- Press `Escape` to return to Normal mode
- Press `v` to enter Visual mode

### Normal Mode Navigation
- Use `h/j/k/l` for left/down/up/right movement
- Use `w/b` to move forward/backward by word
- Use `0` to move to start of line, `$` to move to end
- Use `g` to move to document start, `G` to move to end

### Visual Mode
- All navigation keys will extend selection
- Press `y` to copy (yank) selection
- Press `d` or `x` to cut selection
- Press `c` to cut selection and enter insert mode
- Press `p` to replace selection with clipboard contents

Try it out! Press `i` to start typing.
"#,
        );

        Self { editor }
    }
}

impl eframe::App for VimEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Vim Mode Editor");

                // Display current Vim mode in the UI
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| match self.editor.mode() {
                        EditorMode::Vim(VimMode::Normal) => {
                            ui.label("NORMAL MODE");
                        }
                        EditorMode::Vim(VimMode::Insert) => {
                            ui.label("INSERT MODE");
                        }
                        EditorMode::Vim(VimMode::Visual) => {
                            ui.label("VISUAL MODE");
                        }
                        _ => {
                            ui.label("UNKNOWN MODE");
                        }
                    },
                );
            });

            ui.separator();

            // Show the editor with vim bindings
            self.editor.show(ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ed-Egui Vim Editor",
        native_options,
        Box::new(|_cc| Ok(Box::new(VimEditorApp::default()))),
    )
}
