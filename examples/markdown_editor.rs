use eframe::{egui, epi};
use ed_egui::{EditorWidget, EditingMode};
use ed_egui::syntax::{MarkdownHighlighter, SyntaxHighlighter};

struct MarkdownEditorApp {
    editor: EditorWidget,
    mode_type: ed_egui::modes::EditingModeType,
    preview_pane: bool,
}

impl Default for MarkdownEditorApp {
    fn default() -> Self {
        let editor = EditorWidget::default()
            .with_font_size(16.0)
            .with_line_numbers(true)
            .with_syntax_highlighter(MarkdownHighlighter::new());
            
        Self {
            editor,
            mode_type: ed_egui::modes::EditingModeType::Emacs, // Default mode
            preview_pane: true,
        }
    }
}

impl epi::App for MarkdownEditorApp {
    fn name(&self) -> &str {
        "Ed-Egui Markdown Editor Example"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Markdown Editor");
                
                ui.separator();
                
                ui.radio_value(&mut self.mode_type, ed_egui::modes::EditingModeType::Vim, "Vim Mode");
                ui.radio_value(&mut self.mode_type, ed_egui::modes::EditingModeType::Emacs, "Emacs Mode");
                
                ui.checkbox(&mut self.preview_pane, "Show Preview");
                
                if ui.button("Load Sample").clicked() {
                    self.editor.set_text(sample_text());
                }
            });
        });
        
        // Set the editing mode on the editor widget
        self.editor = std::mem::take(&mut self.editor)
            .with_mode(self.mode_type)
            .with_syntax_highlighter(MarkdownHighlighter::new());
        
        if self.preview_pane {
            // Split the UI into editor and preview panes
            egui::SidePanel::left("editor_panel")
                .resizable(true)
                .min_width(300.0)
                .show(ctx, |ui| {
                    // Use all available space for the editor
                    self.editor.show(ui);
                });
            
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Preview");
                ui.separator();
                
                // Scroll area for the preview
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Simple markdown-like preview (not a full renderer)
                    let text = self.editor.text();
                    let mut in_code_block = false;
                    
                    for line in text.lines() {
                        if line.starts_with("```") {
                            in_code_block = !in_code_block;
                            ui.horizontal(|ui| {
                                ui.monospace(line);
                            });
                            continue;
                        }
                        
                        if in_code_block {
                            ui.horizontal(|ui| {
                                ui.add(egui::TextEdit::multiline(&mut line.to_string())
                                    .code_editor()
                                    .desired_width(f32::INFINITY)
                                    .interactive(false));
                            });
                            continue;
                        }
                        
                        if line.starts_with("# ") {
                            ui.heading(&line[2..]);
                        } else if line.starts_with("## ") {
                            ui.heading(&line[3..]);
                        } else if line.starts_with("### ") {
                            ui.strong(&line[4..]);
                        } else if line.starts_with("- ") {
                            ui.horizontal(|ui| {
                                ui.label("•");
                                ui.label(&line[2..]);
                            });
                        } else {
                            ui.label(line);
                        }
                    }
                });
            });
        } else {
            // Just show the editor in full size
            egui::CentralPanel::default().show(ctx, |ui| {
                self.editor.show(ui);
            });
        }
    }
}

fn sample_text() -> String {
    r#"# Markdown Editor Example

This is a **markdown** editor with _syntax highlighting_ and preview.

## Features

- Full Markdown support
- Embedded code blocks with syntax highlighting
- Live preview pane
- Vim or Emacs key bindings

### Code Examples

Here's some Rust code:

```rust
fn main() {
    println!("Hello, Markdown!");
    
    // A simple loop
    for i in 0..10 {
        if i % 2 == 0 {
            println!("{} is even", i);
        } else {
            println!("{} is odd", i);
        }
    }
}
```

And some Python:

```python
def greet(name):
    """Simple greeting function"""
    return f"Hello, {name}!"

# Test the function
print(greet("Markdown"))
```

## How to Use

1. Write markdown in the editor pane
2. See the preview in real-time
3. Toggle between Vim and Emacs key bindings
4. Hide the preview pane for a distraction-free experience
"#.to_string()
}

fn main() {
    let app = MarkdownEditorApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}