use egui::{Context, InputState};

/// A trait for key handlers that can intercept and process keyboard events
pub trait KeyHandler {
    /// Process keyboard input events before they reach the `TextEdit` widget
    /// Returns a vector of indices to remove from the input events
    fn process_input(&mut self, ctx: &Context, input: &mut InputState) -> Vec<usize>;

    /// Get the name of the key handler
    fn name(&self) -> &'static str;
}
