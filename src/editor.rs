use crossterm::cursor;

use crate::{
    content::Position,
    prelude::{PieceTable, TextTrait},
};

pub struct Editor {
    content: PieceTable,
    pub text: String,
    pub cursor_position: usize,
    temporary_buffer: String,
    temporary_buffer_max_length: usize,
    temporary_buffer_position: usize,
}

impl Editor {
    pub fn new(text: String, temporary_buffer_max_length: usize) -> Self {
        let mut cursor_position = 0; // Start at the end of the text
        if !text.is_empty() {
            // If the text is not empty, set the cursor position to the end of the text
            cursor_position = text.len();
        }

        Self {
            content: PieceTable::new(&text.clone()),
            text,
            temporary_buffer: String::new(),
            temporary_buffer_max_length,
            cursor_position,
            temporary_buffer_position: cursor_position,
        }
    }

    pub fn add_char(&mut self, c: char) {
        if self.temporary_buffer.len() < self.temporary_buffer_max_length {
            self.temporary_buffer.push(c);
            self.text.push(c);
            self.cursor_position += 1;
        }

        // if the buffer is full, we must persist its content to the piece table
        if self.temporary_buffer.len() >= self.temporary_buffer_max_length {
            self.persist_temporary_buffer(true);
        }
    }

    pub fn remove_char(&mut self) {
        if self.cursor_position > 0 {
            // self.content.pop();
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.persist_temporary_buffer(true);
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.total_length() {
            self.cursor_position += 1;
            self.persist_temporary_buffer(true);
        }
    }

    pub fn move_cursor_up(&mut self) {
        // if self.cursor_position.y > 0 {
        //     self.cursor_position.y -= 1;
        // }
    }

    pub fn move_cursor_down(&mut self) {
        // if self.cursor_position.y < self.content.len() as u16 {
        // self.cursor_position.y += 1;
        // }
    }

    pub fn add_new_line(&mut self) {
        // self.content.push('\n');
        // self.cursor_position.x = 0;
        // self.cursor_position.y += 1;
    }

    /// Persists the contents of the temporary buffer to the piece table.
    ///
    /// This function is responsible for flushing the temporary buffer into the main
    /// piece table storage. If the buffer is empty, it does nothing.
    ///
    /// # Parameters
    /// - `force_save`: If `true`, the buffer will be persisted regardless of its current length.
    ///   If `false`, the buffer will only be persisted if its length exceeds half of the
    ///   maximum allowed buffer size. This allows for more efficient batching of edits,
    ///   reducing the number of write operations to the piece table.
    ///
    /// After persisting, the buffer is cleared and the buffer position is updated to the
    /// current cursor position.
    pub fn persist_temporary_buffer(&mut self, force_save: bool) {
        if self.temporary_buffer.is_empty() {
            // If the buffer is empty, we can skip persisting
            return;
        }

        // If the buffer is not empty, we need to persist its content to the piece table
        if force_save || self.temporary_buffer.len() > self.temporary_buffer_max_length / 2 {
            let _ = self
                .content
                .add_text(&self.temporary_buffer.clone(), self.temporary_buffer_position);

            self.temporary_buffer.clear();
            self.temporary_buffer_position = self.cursor_position;
        }
    }
}
