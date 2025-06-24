use crossterm::event::KeyCode;

use crate::prelude::{EnumAddResult, PieceTable, TemporaryBufferAddText, TemporaryBufferDeleteText, TextTrait};

pub struct Editor {
    content: PieceTable,
    pub cursor_position: usize,
    pub temporary_add_buffer: TemporaryBufferAddText,
    pub temporary_delete_buffer: TemporaryBufferDeleteText,
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
            temporary_add_buffer: TemporaryBufferAddText::new(temporary_buffer_max_length, cursor_position),
            temporary_delete_buffer: TemporaryBufferDeleteText::new(temporary_buffer_max_length),
            cursor_position,
        }
    }

    pub fn add_char(&mut self, c: char) {
        use crate::prelude::EnumAddResult;
        
        if !self.temporary_delete_buffer.is_empty() {
            self.persist_delete_buffer();
        }

        let add_result = self.temporary_add_buffer.add_char(c);

        self.cursor_position += 1;

        // Persist the buffer if AddResult::MustPersist is returned
        if let Ok(EnumAddResult::MustPersist) = add_result {
            self.persist_temporary_add_buffer(true);
        }
    }
    
    pub fn get_text(&self) -> String {
        let mut content = self.content.get_text();

        // Insert the temporary buffer at its position if it's not empty
        if !self.temporary_add_buffer.buffer.is_empty() {
            let pos = self.temporary_add_buffer.position;
            content.insert_str(pos, &self.temporary_add_buffer.buffer);
        }
        else if !self.temporary_delete_buffer.is_empty() {
            // If the delete buffer is not empty, we should not show the deleted text
            if let Some((start, end)) = self.temporary_delete_buffer.get_deletion_range() {
                content.replace_range(start..end, "");
            }
        }

        content
    }

    pub fn delete_char(&mut self, key: KeyCode) {
        if self.cursor_position > 0 {
            let deleted_position = self.cursor_position;

            // If the cursor is on the temporary buffer add, remove a character from it
            if !self.temporary_add_buffer.buffer.is_empty() && self.temporary_add_buffer.is_cursor_on_buffer(self.cursor_position) {
                self.temporary_add_buffer.delete_char();
            } else {
                if let Ok(EnumAddResult::MustPersist) = self.temporary_delete_buffer.add_char(deleted_position, key) {
                    // If the delete buffer is full, then delete the text range from the piece table
                    self.persist_delete_buffer();
                }
            }

            if key == KeyCode::Backspace {
                self.cursor_position -= 1; // Move cursor back before deleting with backspace
            }

            self.temporary_add_buffer.update_position(self.cursor_position);
        }
    }

    fn persist_delete_buffer(&mut self) {
        if let Some((start, end)) = self.temporary_delete_buffer.get_deletion_range() {
            let _ = self.content.delete_text(start, end);
            self.temporary_delete_buffer.clear();
        }
    }
    
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.persist_temporary_add_buffer(true);
            self.persist_delete_buffer();
            self.temporary_add_buffer.update_position(self.cursor_position);
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.total_length() {
            self.cursor_position += 1;
            self.persist_temporary_add_buffer(true);
            self.temporary_add_buffer.update_position(self.cursor_position);
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
    pub fn persist_temporary_add_buffer(&mut self, force_save: bool) {
        if self.temporary_add_buffer.buffer.is_empty() {
            // If the buffer is empty, we can skip persisting
            return;
        }

        // If the buffer is not empty, we need to persist its content to the piece table
        if force_save || self.temporary_add_buffer.buffer.len() > self.temporary_add_buffer.max_length / 2 {
            let _ = self
                .content
                .add_text(&self.temporary_add_buffer.buffer.clone(), self.temporary_add_buffer.position);

            self.temporary_add_buffer.clear(self.cursor_position);
        }
    }
}
