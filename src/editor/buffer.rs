/// Core text buffer implementation with cursor
pub struct TextBuffer {
    text: String,
    cursor_pos: usize, // Simple cursor position as character index
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self {
            text: String::new(),
            cursor_pos: 0,
        }
    }
}

impl TextBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.cursor_pos = self.cursor_pos.min(self.text.len());
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_pos
    }

    pub fn set_cursor_position(&mut self, position: usize) {
        self.cursor_pos = position.min(self.text.len());
    }

    // Insert a character at the current cursor position
    pub fn insert_char(&mut self, c: char) {
        self.text.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    // Delete the character before the cursor
    pub fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.text.remove(self.cursor_pos);
        }
    }

    // Delete the character under the cursor
    pub fn delete_char_forward(&mut self) {
        if self.cursor_pos < self.text.len() {
            self.text.remove(self.cursor_pos);
        }
    }

    // Move cursor left
    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    // Move cursor right
    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.text.len() {
            self.cursor_pos += 1;
        }
    }

    // Move to the start of the current line
    pub fn move_to_line_start(&mut self) {
        // Find the previous newline or start of text
        while self.cursor_pos > 0 && !self.text[..self.cursor_pos].ends_with('\n') {
            self.cursor_pos -= 1;
        }

        // If we stopped at a newline, move past it
        if self.cursor_pos > 0 && self.text[..self.cursor_pos].ends_with('\n') {
            self.cursor_pos += 1;
        }
    }

    // Move to the end of the current line
    pub fn move_to_line_end(&mut self) {
        // Find the next newline or end of text
        while self.cursor_pos < self.text.len() && self.text.as_bytes()[self.cursor_pos] != b'\n' {
            self.cursor_pos += 1;
        }
    }

    // Insert a newline at the cursor position
    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
    }
}
