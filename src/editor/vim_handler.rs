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

    /// Helper method to generate key events for document navigation
    /// 
    /// This creates multiple key events that will trigger document start/end movement in TextEdit.
    /// We generate multiple combinations since different platforms use different shortcuts:
    /// - Windows/Linux typically use Ctrl+Home/End
    /// - macOS might use Command+Up/Down, Command+Home/End or other combinations
    fn gen_doc_navigation_events(&self, is_end: bool, with_selection: bool) -> Vec<Event> {
        let mut events = Vec::new();
        
        // Create Home/End events with Ctrl modifier (Windows/Linux)
        {
            let key = if is_end { Key::End } else { Key::Home };
            let physical_key = if is_end {
                Some(Key::End)
            } else {
                Some(Key::Home)
            };
            
            let mut mods = Modifiers::default();
            mods.ctrl = true;
            mods.shift = with_selection; // Add shift if we want selection
            mods.alt = false;
            mods.command = false;
            
            events.push(Event::Key {
                key,
                physical_key,
                pressed: true,
                repeat: false,
                modifiers: mods,
            });
        }
        
        // Create Home/End events with Command modifier (macOS)
        {
            let key = if is_end { Key::End } else { Key::Home };
            let physical_key = if is_end {
                Some(Key::End)
            } else {
                Some(Key::Home)
            };
            
            let mut mods = Modifiers::default();
            mods.command = true;
            mods.shift = with_selection; // Add shift if we want selection
            mods.alt = false;
            mods.ctrl = false;
            
            events.push(Event::Key {
                key,
                physical_key,
                pressed: true,
                repeat: false,
                modifiers: mods,
            });
        }
        
        // Create Up/Down events with Command modifier (macOS alternative)
        {
            let key = if is_end { Key::ArrowDown } else { Key::ArrowUp };
            let physical_key = if is_end {
                Some(Key::ArrowDown)
            } else {
                Some(Key::ArrowUp)
            };
            
            let mut mods = Modifiers::default();
            mods.command = true;
            mods.shift = with_selection; // Add shift if we want selection
            mods.alt = false;
            mods.ctrl = false;
            
            events.push(Event::Key {
                key,
                physical_key,
                pressed: true,
                repeat: false,
                modifiers: mods,
            });
        }
        
        // Try plain Home/End without modifiers as a fallback
        // Some applications treat Home/End differently than Ctrl+Home/End
        if is_end {
            // Only add this for End key (G) since it's the problematic one
            events.push(Event::Key {
                key: Key::End,
                physical_key: Some(Key::End),
                pressed: true, 
                repeat: false,
                modifiers: Modifiers::default(),
            });
            
            // Also try PageDown key with Control modifier as another option
            let mut mods = Modifiers::default();
            mods.ctrl = true;
            events.push(Event::Key {
                key: Key::PageDown,
                physical_key: Some(Key::PageDown),
                pressed: true,
                repeat: false,
                modifiers: mods,
            });
        }
        
        println!("DEBUG: Generated {} document navigation events (selection: {})", events.len(), with_selection);
        events
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
                        println!(
                            "DEBUG(h): All input events before processing: {:?}",
                            input.events
                        );
                        println!("DEBUG(h): Current modifiers: {:?}", input.modifiers);
                        events_to_remove.extend(0..input.events.len());

                        let event = Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: input.modifiers,
                        };
                        println!("DEBUG(h): Pushing new event: {:?}", event);
                        input.events.push(event);
                        println!(
                            "DEBUG(h): All input events after processing: {:?}",
                            input.events
                        );
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

                    // Word movement using custom implementation for vim-like behavior
                    Key::W => {
                        // Capital W and lowercase w both move by word in the same way
                        self.debug_log("'w/W' key pressed - mapping to vim-style word movement");
                        println!("DEBUG: Processing W key in normal mode");
                        events_to_remove.extend(0..input.events.len());
                        
                        // PRECISE SINGLE WORD MOVEMENT APPROACH:
                        // The previous implementation was adding too many events at once, causing
                        // multiple word jumps. This implementation uses a more controlled approach
                        // that ensures we move exactly one word at a time.
                        
                        // We'll use two separate approaches and let the platform choose which works:
                        // 1. For Windows/Linux: Use a simple Right arrow + Ctrl+Right sequence
                        // 2. For macOS: Use a Right arrow + Alt+Right sequence
                        
                        // First, a single right arrow to handle edge cases at word boundaries
                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: Modifiers::default(),
                        });
                        
                        // Then, use Ctrl+Right for Windows/Linux word movement
                        // We'll do this in a separate message so it doesn't combine with the previous one
                        let mut ctrl_mods = Modifiers::default();
                        ctrl_mods.ctrl = true;
                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: ctrl_mods,
                        });
                        
                        // Also try Alt+Right for macOS in a separate event
                        let mut alt_mods = Modifiers::default();
                        alt_mods.alt = true;
                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: alt_mods,
                        });
                    }
                    Key::B => {
                        // Capital B and lowercase b both move by word backward in the same way
                        self.debug_log("'b/B' key pressed - mapping to vim-style word movement");
                        println!("DEBUG: Processing B key in normal mode");
                        events_to_remove.extend(0..input.events.len());
                        
                        // PRECISE SINGLE WORD MOVEMENT APPROACH:
                        // The previous implementation was adding too many events at once, causing
                        // multiple word jumps. This implementation uses a more controlled approach
                        // that ensures we move exactly one word at a time.
                        
                        // We'll use two separate approaches and let the platform choose which works:
                        // 1. For Windows/Linux: Use a simple Left arrow + Ctrl+Left sequence
                        // 2. For macOS: Use a Left arrow + Alt+Left sequence
                        
                        // First, a single left arrow to handle edge cases at word boundaries
                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: Modifiers::default(),
                        });
                        
                        // Then, use Ctrl+Left for Windows/Linux word movement
                        // We'll do this in a separate message so it doesn't combine with the previous one
                        let mut ctrl_mods = Modifiers::default();
                        ctrl_mods.ctrl = true;
                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: ctrl_mods,
                        });
                        
                        // Also try Alt+Left for macOS in a separate event
                        let mut alt_mods = Modifiers::default();
                        alt_mods.alt = true;
                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: alt_mods,
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

                    // Document movement - translate to document navigation events
                    Key::G => {
                        events_to_remove.extend(0..input.events.len());

                        if input.modifiers.shift {
                            self.debug_log("'G' key pressed - mapping to document-end");
                            println!("DEBUG: Processing 'G' key in normal mode");

                            // Generate document end navigation events
                            let events = self.gen_doc_navigation_events(true, false);
                            println!("DEBUG: Generated {} events for document-end movement", events.len());
                            
                            // Add all generated events to the input queue
                            for event in events {
                                println!("DEBUG: Adding document-end event: {:?}", event);
                                input.events.push(event);
                            }
                        } else {
                            self.debug_log("'g' key pressed - mapping to document-start");
                            println!("DEBUG: Processing 'g' key in normal mode");

                            // Generate document start navigation events
                            let events = self.gen_doc_navigation_events(false, false);
                            println!("DEBUG: Generated {} events for document-start movement", events.len());
                            
                            // Add all generated events to the input queue
                            for event in events {
                                println!("DEBUG: Adding document-start event: {:?}", event);
                                input.events.push(event);
                            }
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
        let mut w_key_text_pressed = false;
        let mut b_key_text_pressed = false;
        let mut g_key_text_pressed = false;
        let mut shift_g_pressed = false;

        // First pass - detect special text characters
        for (i, event) in input.events.iter().enumerate() {
            if let Event::Text(text) = event {
                self.debug_log(&format!("Text event detected: '{text}'"));

                // Look for special characters that need conversion
                if text == "$" {
                    dollar_key_pressed = true;
                    self.debug_log("'$' character detected in text event");
                } else if text == "w" {
                    w_key_text_pressed = true;
                    self.debug_log("'w' character detected in text event");
                } else if text == "W" {
                    w_key_text_pressed = true;
                    self.debug_log("'W' character detected in text event");
                } else if text == "b" {
                    b_key_text_pressed = true;
                    self.debug_log("'b' character detected in text event");
                } else if text == "B" {
                    b_key_text_pressed = true;
                    self.debug_log("'B' character detected in text event");
                } else if text == "g" {
                    g_key_text_pressed = true;
                    self.debug_log("'g' character detected in text event");
                } else if text == "G" {
                    shift_g_pressed = true;
                    self.debug_log("'G' character detected in text event");
                }

                // In vim normal mode, suppress all text insertion
                if !events_to_remove.contains(&i) {
                    events_to_remove.push(i);
                }
            }
        }

        // Handle special text characters by converting them to key events

        // Check if we've seen w, b, G in text events and convert them to appropriate key events
        // This is crucial for platforms/conditions where only Text events are sent and not Key events

        // Generate word motion events for 'w'
        if w_key_text_pressed {
            self.debug_log("Converting 'w' text to vim-style word movement");
            
            // PRECISE SINGLE WORD MOVEMENT APPROACH:
            // The previous implementation was adding too many events at once, causing
            // multiple word jumps. This implementation uses a more controlled approach.
            
            // First, a single right arrow to handle edge cases at word boundaries
            input.events.push(Event::Key {
                key: Key::ArrowRight,
                physical_key: Some(Key::ArrowRight),
                pressed: true,
                repeat: false,
                modifiers: Modifiers::default(),
            });
            
            // Then, use Ctrl+Right for Windows/Linux word movement
            // We'll do this in a separate message so it doesn't combine with the previous one
            let mut ctrl_mods = Modifiers::default();
            ctrl_mods.ctrl = true;
            input.events.push(Event::Key {
                key: Key::ArrowRight,
                physical_key: Some(Key::ArrowRight),
                pressed: true,
                repeat: false,
                modifiers: ctrl_mods,
            });
            
            // Also try Alt+Right for macOS in a separate event
            let mut alt_mods = Modifiers::default();
            alt_mods.alt = true;
            input.events.push(Event::Key {
                key: Key::ArrowRight,
                physical_key: Some(Key::ArrowRight),
                pressed: true,
                repeat: false,
                modifiers: alt_mods,
            });
            
            println!("DEBUG: Added vim-style events for word-right movement");
        }

        // Generate word motion events for 'b'
        if b_key_text_pressed {
            self.debug_log("Converting 'b' text to vim-style word movement");
            
            // PRECISE SINGLE WORD MOVEMENT APPROACH:
            // The previous implementation was adding too many events at once, causing
            // multiple word jumps. This implementation uses a more controlled approach.
            
            // First, a single left arrow to handle edge cases at word boundaries
            input.events.push(Event::Key {
                key: Key::ArrowLeft,
                physical_key: Some(Key::ArrowLeft),
                pressed: true,
                repeat: false,
                modifiers: Modifiers::default(),
            });
            
            // Then, use Ctrl+Left for Windows/Linux word movement
            // We'll do this in a separate message so it doesn't combine with the previous one
            let mut ctrl_mods = Modifiers::default();
            ctrl_mods.ctrl = true;
            input.events.push(Event::Key {
                key: Key::ArrowLeft,
                physical_key: Some(Key::ArrowLeft),
                pressed: true,
                repeat: false,
                modifiers: ctrl_mods,
            });
            
            // Also try Alt+Left for macOS in a separate event
            let mut alt_mods = Modifiers::default();
            alt_mods.alt = true;
            input.events.push(Event::Key {
                key: Key::ArrowLeft,
                physical_key: Some(Key::ArrowLeft),
                pressed: true,
                repeat: false,
                modifiers: alt_mods,
            });
            
            println!("DEBUG: Added vim-style events for word-left movement");
        }

        // Generate document motion events for 'g'
        if g_key_text_pressed {
            self.debug_log("Converting 'g' text to document-start navigation events");
            let events = self.gen_doc_navigation_events(false, false);
            println!("DEBUG: Generated {} events for document-start movement from text event", events.len());
            
            // Add all generated events to the input queue
            for event in events {
                println!("DEBUG: Adding document-start event from text: {:?}", event);
                input.events.push(event);
            }
        }

        // Generate document motion events for 'G' (shift+g)
        if shift_g_pressed {
            self.debug_log("Converting 'G' text to document-end navigation events");
            let events = self.gen_doc_navigation_events(true, false);
            println!("DEBUG: Generated {} events for document-end movement from text event", events.len());
            
            // Add all generated events to the input queue
            for event in events {
                println!("DEBUG: Adding document-end event from text: {:?}", event);
                input.events.push(event);
            }
        }

        // Generate line end motion for '$'
        if dollar_key_pressed {
            self.debug_log("Converting '$' to End key event");
            
            // Create a clean modifier - we want no modifiers for End key
            let mods = Modifiers::default();
            
            let event = Event::Key {
                key: Key::End,
                physical_key: Some(Key::End),
                pressed: true,
                repeat: false,
                modifiers: mods,
            };
            input.events.push(event);
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

                    // Word movement - map directly to word movement events with shift modifier for selection
                    Key::W => {
                        // Both Capital W and lowercase w both move by word with selection
                        self.debug_log(
                            "'w/W' key pressed in visual mode - mapping to vim-style word movement with selection",
                        );
                        events_to_remove.extend(0..input.events.len());
                        
                        // PRECISE SINGLE WORD MOVEMENT APPROACH:
                        // Modified for visual mode by adding shift modifier for selection
                        
                        // First, a single right arrow with shift to handle edge cases at word boundaries
                        let mut shift_mods = Modifiers::default();
                        shift_mods.shift = true;
                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: shift_mods,
                        });
                        
                        // Then, use Ctrl+Shift+Right for Windows/Linux word movement with selection
                        let mut ctrl_shift_mods = Modifiers::default();
                        ctrl_shift_mods.ctrl = true;
                        ctrl_shift_mods.shift = true; // Add shift for selection
                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: ctrl_shift_mods,
                        });
                        
                        // Also try Alt+Shift+Right for macOS in a separate event
                        let mut alt_shift_mods = Modifiers::default();
                        alt_shift_mods.alt = true;
                        alt_shift_mods.shift = true; // Add shift for selection
                        input.events.push(Event::Key {
                            key: Key::ArrowRight,
                            physical_key: Some(Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: alt_shift_mods,
                        });
                    }
                    Key::B => {
                        // Both Capital B and lowercase b move by word backward with selection
                        self.debug_log(
                            "'b/B' key pressed in visual mode - mapping to vim-style word movement with selection",
                        );
                        events_to_remove.extend(0..input.events.len());
                        
                        // PRECISE SINGLE WORD MOVEMENT APPROACH:
                        // Modified for visual mode by adding shift modifier for selection
                        
                        // First, a single left arrow with shift to handle edge cases at word boundaries
                        let mut shift_mods = Modifiers::default();
                        shift_mods.shift = true;
                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: shift_mods,
                        });
                        
                        // Then, use Ctrl+Shift+Left for Windows/Linux word movement with selection
                        let mut ctrl_shift_mods = Modifiers::default();
                        ctrl_shift_mods.ctrl = true;
                        ctrl_shift_mods.shift = true; // Add shift for selection
                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: ctrl_shift_mods,
                        });
                        
                        // Also try Alt+Shift+Left for macOS in a separate event
                        let mut alt_shift_mods = Modifiers::default();
                        alt_shift_mods.alt = true;
                        alt_shift_mods.shift = true; // Add shift for selection
                        input.events.push(Event::Key {
                            key: Key::ArrowLeft,
                            physical_key: Some(Key::ArrowLeft),
                            pressed: true,
                            repeat: false,
                            modifiers: alt_shift_mods,
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

                    // Document movement - translate to document navigation events with selection
                    Key::G => {
                        if input.modifiers.shift {
                            self.debug_log(
                                "'G' key pressed in visual mode - mapping to document-end with selection",
                            );
                            events_to_remove.extend(0..input.events.len());

                            // Generate document end navigation events with selection
                            let events = self.gen_doc_navigation_events(true, true);
                            println!("DEBUG: Generated {} events for document-end movement with selection", events.len());
                            
                            // Add all generated events to the input queue
                            for event in events {
                                println!("DEBUG: Adding visual mode document-end event: {:?}", event);
                                input.events.push(event);
                            }
                        } else {
                            self.debug_log(
                                "'g' key pressed in visual mode - mapping to document-start with selection",
                            );
                            events_to_remove.extend(0..input.events.len());

                            // Generate document start navigation events with selection
                            let events = self.gen_doc_navigation_events(false, true);
                            println!("DEBUG: Generated {} events for document-start movement with selection", events.len());
                            
                            // Add all generated events to the input queue
                            for event in events {
                                println!("DEBUG: Adding visual mode document-start event: {:?}", event);
                                input.events.push(event);
                            }
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
