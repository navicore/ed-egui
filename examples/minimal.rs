use eframe::egui;

struct MinimalEditorApp {
    text: String,
    cursor_pos: usize,
}

impl Default for MinimalEditorApp {
    fn default() -> Self {
        Self {
            text: "Hello, ed-egui!".to_string(),
            cursor_pos: 0,
        }
    }
}

impl eframe::App for MinimalEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Minimal Editor Test");
            ui.label("This is a basic test of the editor integration with egui.");
            
            // Use egui's built-in TextEdit while we develop our own editor
            let response = ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(10)
                    .font(egui::TextStyle::Monospace)
            );
            
            // Display editor state for debugging
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Text length:");
                ui.label(format!("{}", self.text.len()));
                
                if response.has_focus() {
                    ui.label("Editor has focus");
                }
            });
            
            // Display information about key presses
            if response.has_focus() {
                ui.separator();
                ui.label("Key presses:");
                
                let input = ui.input(|i| i.clone());
                for key in &input.keys_down {
                    if input.key_pressed(*key) {
                        ui.label(format!("Key pressed: {:?}", key));
                    }
                }
                
                for event in &input.events {
                    if let egui::Event::Text(text) = event {
                        ui.label(format!("Text entered: {:?}", text));
                    }
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Minimal Ed-Egui Test",
        native_options,
        Box::new(|_cc| Box::new(MinimalEditorApp::default()))
    )
}