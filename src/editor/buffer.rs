/// Core text buffer implementation with cursor
#[derive(Default)]
pub struct TextBuffer {
    /// The text content of the buffer
    text: String,
    /// The current cursor position in the text
    cursor_pos: usize, // Character index
    /// The current line positions (cached for efficiency)
    line_positions: Vec<usize>,
    /// Whether the line positions need to be recalculated
    needs_line_update: bool,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor_pos: 0,
            line_positions: vec![0],
            needs_line_update: false,
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn text(&self) -> &str {
        &self.text
    }

    pub const fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.cursor_pos = self.cursor_pos.min(self.text.len());
        self.needs_line_update = true;
    }

    pub const fn cursor_position(&self) -> usize {
        self.cursor_pos
    }

    pub fn set_cursor_position(&mut self, position: usize) {
        self.cursor_pos = position.min(self.text.len());
    }

    // Insert a character at the current cursor position
    pub fn insert_char(&mut self, c: char) {
        self.text.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
        self.needs_line_update = true;
    }

    // Delete the character before the cursor
    pub fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.text.remove(self.cursor_pos);
            self.needs_line_update = true;
        }
    }

    // Delete the character under the cursor
    pub fn delete_char_forward(&mut self) {
        if self.cursor_pos < self.text.len() {
            self.text.remove(self.cursor_pos);
            self.needs_line_update = true;
        }
    }

    // NOTE: All cursor movement functionality has been removed and is now
    // handled directly by the TextEdit widget. The cursor_pos field in this
    // struct is only updated from the TextEdit widget's cursor position.

    // Insert a newline at the cursor position
    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
        self.needs_line_update = true;
    }

    /// Calculate positions of all line starts
    fn update_line_positions(&mut self) {
        if !self.needs_line_update {
            return;
        }

        self.line_positions.clear();
        self.line_positions.push(0); // First line always starts at position 0

        for (i, c) in self.text.char_indices() {
            if c == '\n' {
                self.line_positions.push(i + 1); // Line starts after the newline
            }
        }

        self.needs_line_update = false;
    }

    /// Get the current line number (0-based)
    pub fn current_line(&mut self) -> usize {
        self.update_line_positions();
        let pos = self.cursor_pos;

        // Find the last line start position that's less than or equal to cursor_pos
        match self.line_positions.binary_search(&pos) {
            Ok(exact_match) => exact_match, // cursor is exactly at line start
            Err(insertion_point) => insertion_point - 1, // cursor is in the middle of a line
        }
    }

    /// Get the current column (0-based)
    pub fn current_column(&mut self) -> usize {
        self.update_line_positions();
        let line = self.current_line();
        let line_start = self.line_positions[line];
        self.cursor_pos - line_start
    }

    /// Get the number of lines in the buffer
    pub fn line_count(&mut self) -> usize {
        self.update_line_positions();
        self.line_positions.len()
    }

    // Line and column information functions are still useful for status bar display
    // but no longer directly manipulate the cursor position

    // NOTE: All cursor movement functionality has been removed and is now
    // handled directly by the TextEdit widget. The cursor_pos field in this
    // struct is only updated from the TextEdit widget's cursor position.
}
