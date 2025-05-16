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

    // Move cursor left
    pub const fn move_cursor_left(&mut self) {
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

    /// Move cursor to a specific line and column
    pub fn move_cursor_to(&mut self, line: usize, column: usize) {
        self.update_line_positions();

        // Clamp line to valid range
        let line = line.min(self.line_positions.len() - 1);

        // Get line start position
        let line_start = self.line_positions[line];

        // Get line end position
        let line_end = if line < self.line_positions.len() - 1 {
            self.line_positions[line + 1] - 1 // -1 to account for the newline
        } else {
            self.text.len()
        };

        // Calculate max column position for this line
        let max_column = line_end - line_start;

        // Set cursor position to line start + column (clamped to line length)
        self.cursor_pos = line_start + column.min(max_column);
    }

    // Move cursor up one line, trying to maintain the same column position
    pub fn move_cursor_up(&mut self) {
        self.update_line_positions();
        let current_line = self.current_line();

        if current_line > 0 {
            let current_column = self.current_column();
            self.move_cursor_to(current_line - 1, current_column);
        }
    }

    // Move cursor down one line, trying to maintain the same column position
    pub fn move_cursor_down(&mut self) {
        self.update_line_positions();
        let current_line = self.current_line();

        if current_line < self.line_count() - 1 {
            let current_column = self.current_column();
            self.move_cursor_to(current_line + 1, current_column);
        }
    }

    // Move cursor to the beginning of the word or the previous word
    pub fn move_cursor_word_left(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }

        // First, skip any whitespace to the left
        while self.cursor_pos > 0
            && self.text[self.cursor_pos - 1..].starts_with(|c: char| c.is_whitespace())
        {
            self.cursor_pos -= 1;
        }

        // Then, move to the beginning of the current word
        while self.cursor_pos > 0
            && !self.text[self.cursor_pos - 1..].starts_with(|c: char| c.is_whitespace())
        {
            self.cursor_pos -= 1;
        }
    }

    // Move cursor to the beginning of the next word
    pub fn move_cursor_word_right(&mut self) {
        if self.cursor_pos >= self.text.len() {
            return;
        }

        // First, move to the end of the current word
        while self.cursor_pos < self.text.len()
            && !self.text[self.cursor_pos..].starts_with(|c: char| c.is_whitespace())
        {
            self.cursor_pos += 1;
        }

        // Then, skip any whitespace
        while self.cursor_pos < self.text.len()
            && self.text[self.cursor_pos..].starts_with(|c: char| c.is_whitespace())
        {
            self.cursor_pos += 1;
        }
    }

    // Move cursor to the beginning of the buffer
    pub fn move_cursor_document_start(&mut self) {
        self.cursor_pos = 0;
    }

    // Move cursor to the end of the buffer
    pub fn move_cursor_document_end(&mut self) {
        self.cursor_pos = self.text.len();
    }
}
