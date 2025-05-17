use crate::editor::commands::VimMode;
use crate::editor::keyhandler::KeyHandler;
use egui::{Context, Event, InputState, Key, Modifiers};

/// Implements Vim key handling for the editor
pub struct VimKeyHandler {
    /// The current vim mode (Normal, Insert, Visual)
    mode: VimMode,
    /// Debug printing enabled/disabled
    debug: bool,
}

impl Default for VimKeyHandler {
    fn default() -> Self {
        Self {
            mode: VimMode::Normal,
            debug: false,
        }
    }
}

impl VimKeyHandler {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub const fn mode(&self) -> VimMode {
        self.mode
    }

    /// Set the current vim mode
    pub const fn set_mode(&mut self, mode: VimMode) {
        self.mode = mode;
    }

    /// Enable or disable debug logging
    fn debug_log(&self, message: &str) {
        if self.debug {
            println!("[VimKeyHandler] {message}");
        }
    }

    /// Toggle visual mode from normal mode, or go to normal mode from visual mode
    fn toggle_visual_mode(&mut self) {
        match self.mode {
            VimMode::Normal => {
                self.debug_log("Entering visual mode");
                self.mode = VimMode::Visual;
            }
            VimMode::Visual => {
                self.debug_log("Exiting visual mode");
                self.mode = VimMode::Normal;
            }
            VimMode::Insert => {
                // Only toggle between normal and visual
                self.debug_log("Cannot toggle visual mode from current mode");
            }
        }
    }

    /// Handle the key events for vim normal mode
    #[allow(clippy::too_many_lines)]
    fn handle_normal_mode(&mut self, input: &mut InputState) -> Vec<usize> {
        let mut events_to_remove = Vec::new();

        // Process keyboard events (individual keys)
        for key in &input.keys_down {
            if input.key_pressed(*key) {
                match *key {
                    // Mode transitions
                    Key::I => {
                        self.debug_log("'i' key pressed - entering insert mode");
                        self.mode = VimMode::Insert;
                        events_to_remove.extend(0..input.events.len());
                        break;
                    }
                    Key::V => {
                        self.debug_log("'v' key pressed - entering visual mode");
                        self.toggle_visual_mode();
                        events_to_remove.extend(0..input.events.len());
                        break;
                    }

                    // Basic movement - translate to arrow keys
                    Key::H => {
                        self.debug_log("'h' key pressed - mapping to Left arrow");
                        events_to_remove.extend(0..input.events.len());

                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: input.modifiers,
                        });
                    }
                    Key::J => {
                        self.debug_log("'j' key pressed - mapping to Down arrow");
                        events_to_remove.extend(0..input.events.len());

                        input.events.push(Event::Key {
                            key: Key::ArrowDown,
                            physical_key: Some(Key::ArrowDown),
                            pressed: true,
                            repeat: false,
                            modifiers: input.modifiers,
                        });
                    }
                    Key::K => {
                        self.debug_log("'k' key pressed - mapping to Up arrow");
                        events_to_remove.extend(0..input.events.len());

                        input.events.push(Event::Key {
                            key: Key::ArrowUp,
                            physical_key: Some(Key::ArrowUp),
                            pressed: true,
                            repeat: false,
                            modifiers: input.modifiers,
                        });
                    }
                    Key::L => {
                        self.debug_log("'l' key pressed - mapping to Right arrow");
                        events_to_remove.extend(0..input.events.len());

                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: input.modifiers,
                        });
                    }

                    // Word movement - translate to Ctrl+Arrow keys
                    Key::W => {
                        self.debug_log("'w' key pressed - mapping to Ctrl+Right");
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.ctrl = true;

                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }
                    Key::B => {
                        self.debug_log("'b' key pressed - mapping to Ctrl+Left");
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.ctrl = true;

                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }

                    // Line movement - translate to Home/End keys
                    Key::Num0 => {
                        self.debug_log("'0' key pressed - mapping to Home key");
                        events_to_remove.extend(0..input.events.len());

                        input.events.push(Event::Key {
                            key: Key::Home,
                            physical_key: Some(Key::Home),
                            pressed: true,
                            repeat: false,
                            modifiers: input.modifiers,
                        });
                    }
                    Key::Num4 if input.modifiers.shift => {
                        self.debug_log("'$' key pressed (Shift+4) - mapping to End key");
                        events_to_remove.extend(0..input.events.len());

                        input.events.push(Event::Key {
                            key: Key::End,
                            physical_key: Some(Key::End),
                            pressed: true,
                            repeat: false,
                            modifiers: Modifiers::default(), // Remove shift
                        });
                    }

                    // Document movement - translate to Ctrl+Home/End
                    Key::G => {
                        if input.modifiers.shift {
                            self.debug_log("'G' key pressed - mapping to Ctrl+End");
                            events_to_remove.extend(0..input.events.len());

                            let mut mods = input.modifiers;
                            mods.ctrl = true;
                            mods.shift = false;

                            input.events.push(Event::Key {
                                key: Key::End,
                                physical_key: Some(Key::End),
                                pressed: true,
                                repeat: false,
                                modifiers: mods,
                            });
                        } else {
                            self.debug_log("'g' key pressed - mapping to Ctrl+Home");
                            events_to_remove.extend(0..input.events.len());

                            let mut mods = input.modifiers;
                            mods.ctrl = true;

                            input.events.push(Event::Key {
                                key: Key::Home,
                                physical_key: Some(Key::Home),
                                pressed: true,
                                repeat: false,
                                modifiers: mods,
                            });
                        }
                    }

                    // Editing operations
                    Key::X => {
                        self.debug_log("'x' key pressed - mapping to Delete");
                        events_to_remove.extend(0..input.events.len());

                        input.events.push(Event::Key {
                            key: Key::Delete,
                            physical_key: Some(Key::Delete),
                            pressed: true,
                            repeat: false,
                            modifiers: input.modifiers,
                        });
                    }

                    _ => {}
                }
            }
        }

        // Handle text events in normal mode
        let mut dollar_key_pressed = false;

        // First pass - detect special text characters
        for (i, event) in input.events.iter().enumerate() {
            if let Event::Text(text) = event {
                if self.debug {
                    println!("[VimKeyHandler] Text event: '{text}'");
                }

                // Look for special characters that need conversion
                if text == "$" {
                    dollar_key_pressed = true;
                    self.debug_log("'$' character detected in text event");
                }

                // In vim normal mode, suppress all text insertion
                if !events_to_remove.contains(&i) {
                    events_to_remove.push(i);
                }
            }
        }

        // Handle special text characters by converting them to key events
        if dollar_key_pressed {
            self.debug_log("Converting $ to End key event");

            input.events.push(Event::Key {
                key: Key::End,
                physical_key: Some(Key::End),
                pressed: true,
                repeat: false,
                modifiers: Modifiers::default(),
            });
        }

        events_to_remove
    }

    /// Handle the key events for vim insert mode
    fn handle_insert_mode(&mut self, input: &InputState) -> Vec<usize> {
        let mut events_to_remove = Vec::new();

        // Check for Escape key to exit insert mode
        for key in &input.keys_down {
            if *key == Key::Escape && input.key_pressed(*key) {
                self.debug_log("Escape key pressed - exiting insert mode");
                self.mode = VimMode::Normal;
                events_to_remove.extend(0..input.events.len());
                break;
            }
        }

        events_to_remove
    }

    /// Handle the key events for vim visual mode
    #[allow(clippy::too_many_lines)]
    fn handle_visual_mode(&mut self, input: &mut InputState) -> Vec<usize> {
        let mut events_to_remove = Vec::new();

        // Process keyboard events (individual keys)
        for key in &input.keys_down {
            if input.key_pressed(*key) {
                match *key {
                    // Exit visual mode with Escape
                    Key::Escape => {
                        self.debug_log("Escape key pressed - exiting visual mode");
                        self.mode = VimMode::Normal;
                        events_to_remove.extend(0..input.events.len());
                        break;
                    }

                    // Basic movement - translate to Shift+arrow keys to create selection
                    Key::H => {
                        self.debug_log(
                            "'h' key pressed in visual mode - mapping to Shift+Left arrow",
                        );
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.shift = true;

                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }
                    Key::J => {
                        self.debug_log(
                            "'j' key pressed in visual mode - mapping to Shift+Down arrow",
                        );
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.shift = true;

                        input.events.push(Event::Key {
                            key: Key::ArrowDown,
                            physical_key: Some(Key::ArrowDown),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }
                    Key::K => {
                        self.debug_log(
                            "'k' key pressed in visual mode - mapping to Shift+Up arrow",
                        );
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.shift = true;

                        input.events.push(Event::Key {
                            key: Key::ArrowUp,
                            physical_key: Some(Key::ArrowUp),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }
                    Key::L => {
                        self.debug_log(
                            "'l' key pressed in visual mode - mapping to Shift+Right arrow",
                        );
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.shift = true;

                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }

                    // Word movement - translate to Shift+Ctrl+Arrow keys
                    Key::W => {
                        self.debug_log(
                            "'w' key pressed in visual mode - mapping to Shift+Ctrl+Right",
                        );
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.ctrl = true;
                        mods.shift = true;

                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }
                    Key::B => {
                        self.debug_log(
                            "'b' key pressed in visual mode - mapping to Shift+Ctrl+Left",
                        );
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.ctrl = true;
                        mods.shift = true;

                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }

                    // Line movement - translate to Shift+Home/End keys
                    Key::Num0 => {
                        self.debug_log(
                            "'0' key pressed in visual mode - mapping to Shift+Home key",
                        );
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.shift = true;

                        input.events.push(Event::Key {
                            key: Key::Home,
                            physical_key: Some(Key::Home),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }
                    Key::Num4 if input.modifiers.shift => {
                        self.debug_log(
                            "'$' key pressed (Shift+4) in visual mode - mapping to Shift+End",
                        );
                        events_to_remove.extend(0..input.events.len());

                        // Keep shift for selection, but remove it from the $ character
                        let mut mods = Modifiers {
                            alt: core::default::Default::default(),
                            ctrl: core::default::Default::default(),
                            shift: core::default::Default::default(),
                            mac_cmd: core::default::Default::default(),
                            command: core::default::Default::default(),
                        };
                        mods.shift = true;

                        input.events.push(Event::Key {
                            key: Key::End,
                            physical_key: Some(Key::End),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });
                    }

                    // Document movement - translate to Shift+Ctrl+Home/End
                    Key::G => {
                        if input.modifiers.shift {
                            self.debug_log(
                                "'G' key pressed in visual mode - mapping to Shift+Ctrl+End",
                            );
                            events_to_remove.extend(0..input.events.len());

                            let mut mods = input.modifiers;
                            mods.ctrl = true;
                            // Keep shift for selection

                            input.events.push(Event::Key {
                                key: Key::End,
                                physical_key: Some(Key::End),
                                pressed: true,
                                repeat: false,
                                modifiers: mods,
                            });
                        } else {
                            self.debug_log(
                                "'g' key pressed in visual mode - mapping to Shift+Ctrl+Home",
                            );
                            events_to_remove.extend(0..input.events.len());

                            let mut mods = input.modifiers;
                            mods.ctrl = true;
                            mods.shift = true;

                            input.events.push(Event::Key {
                                key: Key::Home,
                                physical_key: Some(Key::Home),
                                pressed: true,
                                repeat: false,
                                modifiers: mods,
                            });
                        }
                    }

                    // Delete/cut selection with x or d
                    Key::X | Key::D => {
                        self.debug_log(
                            "'x' or 'd' key pressed in visual mode - mapping to Ctrl+X (cut)",
                        );
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.ctrl = true;

                        input.events.push(Event::Key {
                            key: Key::X,
                            physical_key: Some(Key::X),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });

                        // Exit visual mode after operation
                        self.mode = VimMode::Normal;
                    }

                    // Copy selection with y (yank)
                    Key::Y => {
                        self.debug_log("'y' key pressed in visual mode - mapping to Ctrl+C (copy)");
                        events_to_remove.extend(0..input.events.len());

                        let mut mods = input.modifiers;
                        mods.ctrl = true;

                        input.events.push(Event::Key {
                            key: Key::C,
                            physical_key: Some(Key::C),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });

                        // Exit visual mode after operation
                        self.mode = VimMode::Normal;
                    }

                    // Change selection with c (cut + enter insert mode)
                    Key::C => {
                        self.debug_log(
                            "'c' key pressed in visual mode - cut and enter insert mode",
                        );
                        events_to_remove.extend(0..input.events.len());

                        // First do a cut operation (Ctrl+X)
                        let mut mods = input.modifiers;
                        mods.ctrl = true;

                        input.events.push(Event::Key {
                            key: Key::X,
                            physical_key: Some(Key::X),
                            pressed: true,
                            repeat: false,
                            modifiers: mods,
                        });

                        // Then enter insert mode
                        self.mode = VimMode::Insert;
                    }

                    // Paste over selection with p
                    Key::P => {
                        self.debug_log("'p' key pressed in visual mode - cut and paste");
                        events_to_remove.extend(0..input.events.len());

                        // First do a cut operation (Ctrl+X)
                        let mut cut_mods = input.modifiers;
                        cut_mods.ctrl = true;

                        input.events.push(Event::Key {
                            key: Key::X,
                            physical_key: Some(Key::X),
                            pressed: true,
                            repeat: false,
                            modifiers: cut_mods,
                        });

                        // Then do a paste operation (Ctrl+V)
                        let mut paste_mods = input.modifiers;
                        paste_mods.ctrl = true;

                        input.events.push(Event::Key {
                            key: Key::V,
                            physical_key: Some(Key::V),
                            pressed: true,
                            repeat: false,
                            modifiers: paste_mods,
                        });

                        // Exit visual mode after operation
                        self.mode = VimMode::Normal;
                    }

                    _ => {}
                }
            }
        }

        // Suppress all text insertion in visual mode
        for (i, event) in input.events.iter().enumerate() {
            if let Event::Text(_) = event {
                if !events_to_remove.contains(&i) {
                    events_to_remove.push(i);
                }
            }
        }

        events_to_remove
    }
}

impl KeyHandler for VimKeyHandler {
    fn process_input(&mut self, _ctx: &Context, input: &mut InputState) -> Vec<usize> {
        match self.mode {
            VimMode::Normal => self.handle_normal_mode(input),
            VimMode::Insert => self.handle_insert_mode(input),
            VimMode::Visual => self.handle_visual_mode(input),
        }
    }

    fn name(&self) -> &'static str {
        "vim"
    }
}
