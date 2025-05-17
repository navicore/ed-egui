use crate::editor::commands::EditorCommand;
use crate::editor::keyhandler::KeyHandler;
use egui::{Context, Event, InputState, Key, Modifiers};

/// Implements Emacs key handling for the editor

#[derive(Default)]
pub struct EmacsKeyHandler {
    /// Debug printing enabled/disabled
    debug: bool,
    /// Commands that need to be executed
    /// NOTE: This field is no longer used as we now generate TextEdit-compatible events directly
    /// Kept for backward compatibility, but may be removed in the future
    pub commands: Vec<EditorCommand>,
}

impl EmacsKeyHandler {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Enable or disable debug logging
    fn debug_log(&self, message: &str) {
        if self.debug {
            println!("[EmacsKeyHandler] {message}");
        }
    }

    /// Clear any commands that have been queued up
    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }
}

impl KeyHandler for EmacsKeyHandler {
    fn process_input(&mut self, _ctx: &Context, input: &mut InputState) -> Vec<usize> {
        // Clear any previous commands
        self.commands.clear();

        // Emacs uses control and alt key combinations for most commands
        // We'll replace them with TextEdit-compatible events
        let mut events_to_remove = Vec::new();

        // Process CTRL key combinations
        if input.modifiers.ctrl {
            // Basic movement - map to arrow keys
            if input.key_pressed(Key::F) {
                self.debug_log("Ctrl+F pressed - mapping to Right arrow");
                events_to_remove.extend(0..input.events.len());

                input.events.push(Event::Key {
                    key: Key::ArrowRight,
                    physical_key: Some(Key::ArrowRight),
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::default(),
                });
            }
            if input.key_pressed(Key::B) {
                self.debug_log("Ctrl+B pressed - mapping to Left arrow");
                events_to_remove.extend(0..input.events.len());

                input.events.push(Event::Key {
                    key: Key::ArrowLeft,
                    physical_key: Some(Key::ArrowLeft),
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::default(),
                });
            }
            if input.key_pressed(Key::P) {
                self.debug_log("Ctrl+P pressed - mapping to Up arrow");
                events_to_remove.extend(0..input.events.len());

                input.events.push(Event::Key {
                    key: Key::ArrowUp,
                    physical_key: Some(Key::ArrowUp),
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::default(),
                });
            }
            if input.key_pressed(Key::N) {
                self.debug_log("Ctrl+N pressed - mapping to Down arrow");
                events_to_remove.extend(0..input.events.len());

                input.events.push(Event::Key {
                    key: Key::ArrowDown,
                    physical_key: Some(Key::ArrowDown),
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::default(),
                });
            }

            // Line movement - map to Home/End keys
            if input.key_pressed(Key::A) {
                self.debug_log("Ctrl+A pressed - mapping to Home key");
                events_to_remove.extend(0..input.events.len());

                input.events.push(Event::Key {
                    key: Key::Home,
                    physical_key: Some(Key::Home),
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::default(),
                });
            }
            if input.key_pressed(Key::E) {
                self.debug_log("Ctrl+E pressed - mapping to End key");
                events_to_remove.extend(0..input.events.len());

                input.events.push(Event::Key {
                    key: Key::End,
                    physical_key: Some(Key::End),
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::default(),
                });
            }

            // Document movement - map to Ctrl+Home/Ctrl+End
            if input.key_pressed(Key::Home) {
                self.debug_log("Ctrl+Home pressed - document start");
                events_to_remove.extend(0..input.events.len());
                
                // Keep the Ctrl modifier
                input.events.push(Event::Key {
                    key: Key::Home,
                    physical_key: Some(Key::Home),
                    pressed: true,
                    repeat: false,
                    modifiers: input.modifiers,
                });
            }
            if input.key_pressed(Key::End) {
                self.debug_log("Ctrl+End pressed - document end");
                events_to_remove.extend(0..input.events.len());
                
                // Keep the Ctrl modifier
                input.events.push(Event::Key {
                    key: Key::End,
                    physical_key: Some(Key::End),
                    pressed: true,
                    repeat: false,
                    modifiers: input.modifiers,
                });
            }
        }

        // Process ALT (Meta) key combinations
        if input.modifiers.alt {
            // Word movement - map to Ctrl+Left/Ctrl+Right (TextEdit standard)
            if input.key_pressed(Key::F) {
                self.debug_log("Alt+F pressed - mapping to Ctrl+Right");
                events_to_remove.extend(0..input.events.len());

                let mut mods = Modifiers::default();
                mods.ctrl = true;

                input.events.push(Event::Key {
                    key: Key::ArrowRight,
                    physical_key: Some(Key::ArrowRight),
                    pressed: true,
                    repeat: false,
                    modifiers: mods,
                });
            }
            if input.key_pressed(Key::B) {
                self.debug_log("Alt+B pressed - mapping to Ctrl+Left");
                events_to_remove.extend(0..input.events.len());

                let mut mods = Modifiers::default();
                mods.ctrl = true;

                input.events.push(Event::Key {
                    key: Key::ArrowLeft,
                    physical_key: Some(Key::ArrowLeft),
                    pressed: true,
                    repeat: false,
                    modifiers: mods,
                });
            }

            // Document movement
            if input.key_pressed(Key::Comma) && input.modifiers.shift {
                self.debug_log("Alt+< pressed - mapping to Ctrl+Home");
                events_to_remove.extend(0..input.events.len());

                let mut mods = Modifiers::default();
                mods.ctrl = true;

                input.events.push(Event::Key {
                    key: Key::Home,
                    physical_key: Some(Key::Home),
                    pressed: true,
                    repeat: false,
                    modifiers: mods,
                });
            }
            if input.key_pressed(Key::Period) && input.modifiers.shift {
                self.debug_log("Alt+> pressed - mapping to Ctrl+End");
                events_to_remove.extend(0..input.events.len());

                let mut mods = Modifiers::default();
                mods.ctrl = true;

                input.events.push(Event::Key {
                    key: Key::End,
                    physical_key: Some(Key::End),
                    pressed: true,
                    repeat: false,
                    modifiers: mods,
                });
            }
        }

        events_to_remove
    }

    fn name(&self) -> &'static str {
        "emacs"
    }
}
