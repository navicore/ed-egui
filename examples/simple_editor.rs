use eframe::{egui, epi};
use ed_egui::{EditorWidget, EditingMode};

struct EditorApp {
    editor: EditorWidget,
    mode_type: ed_egui::modes::EditingModeType,
}

impl Default for EditorApp {
    fn default() -> Self {
        let editor = EditorWidget::default()
            .with_font_size(16.0)
            .with_line_numbers(true);
            
        Self {
            editor,
            mode_type: ed_egui::modes::EditingModeType::Emacs, // Default mode
        }
    }
}

impl epi::App for EditorApp {
    fn name(&self) -> &str {
        "Ed-Egui Editor Example"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Ed-Egui Example");
                
                ui.separator();
                
                ui.radio_value(&mut self.mode_type, ed_egui::modes::EditingModeType::Vim, "Vim Mode");
                ui.radio_value(&mut self.mode_type, ed_egui::modes::EditingModeType::Emacs, "Emacs Mode");
                
                if ui.button("Load Sample").clicked() {
                    self.editor.set_text(sample_text());
                }
            });
            
            ui.separator();
            
            // Set the editing mode on the editor widget
            self.editor = std::mem::take(&mut self.editor).with_mode(self.mode_type);
            
            // Use all available space for the editor
            self.editor.show(ui);
        });
    }
}

fn sample_text() -> String {
    r#"# Ed-Egui Editor

This is a sample document to demonstrate the editor's capabilities.

## Features

- Vim and Emacs key bindings
- Syntax highlighting for markdown
- Code blocks with language-specific highlighting

## Code Example

```rust
fn main() {
    println!("Hello, world!");
    
    // This is a comment
    let x = 42;
    let s = String::from("Hello");
}
```

## More Text

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor 
incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud 
exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
"#.to_string()
}

fn main() {
    let app = EditorApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}