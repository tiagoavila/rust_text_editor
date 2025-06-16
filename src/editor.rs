use crate::prelude::{EnumAddResult, PieceTable, TemporaryBufferAddText, TemporaryBufferDeleteText, TextTrait};

pub struct Editor {
    content: PieceTable,
    pub cursor_position: usize,
    pub temporary_buffer_add: TemporaryBufferAddText,
    pub temporary_buffer_delete: TemporaryBufferDeleteText,
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
            temporary_buffer_add: TemporaryBufferAddText::new(temporary_buffer_max_length, cursor_position),
            temporary_buffer_delete: TemporaryBufferDeleteText::new(temporary_buffer_max_length),
            cursor_position,
        }
    }

    pub fn add_char(&mut self, c: char) {
        use crate::prelude::EnumAddResult;

        let add_result = self.temporary_buffer_add.add_char(c);

        self.cursor_position += 1;

        // Persist the buffer if AddResult::MustPersist is returned
        if let Ok(EnumAddResult::MustPersist) = add_result {
            self.persist_temporary_buffer(true);
        }
    }
    
    pub fn get_text(&self) -> String {
        let mut content = self.content.get_text();

        // Insert the temporary buffer at its position if it's not empty
        if !self.temporary_buffer_add.buffer.is_empty() {
            let pos = self.temporary_buffer_add.position;
            content.insert_str(pos, &self.temporary_buffer_add.buffer);
        }

        content
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let deleted_position = self.cursor_position;
            self.cursor_position -= 1;

            // If the cursor is on the temporary buffer add, remove a character from it
            if !self.temporary_buffer_add.buffer.is_empty() && self.temporary_buffer_add.is_cursor_on_buffer(self.cursor_position) {
                self.temporary_buffer_add.delete_char();
            } else {
                if let Ok(EnumAddResult::MustPersist) = self.temporary_buffer_delete.add_char(deleted_position) {
                    // If the delete buffer is full, then delete the text range from the piece table
                    if let Some((start, end)) = self.temporary_buffer_delete.get_deletion_range() {
                        let _ = self.content.delete_text(start, end);
                    }
                }
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.persist_temporary_buffer(true);
            self.temporary_buffer_add.update_position(self.cursor_position);
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.total_length() {
            self.cursor_position += 1;
            self.persist_temporary_buffer(true);
            self.temporary_buffer_add.update_position(self.cursor_position);
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
        if self.temporary_buffer_add.buffer.is_empty() {
            // If the buffer is empty, we can skip persisting
            return;
        }

        // If the buffer is not empty, we need to persist its content to the piece table
        if force_save || self.temporary_buffer_add.buffer.len() > self.temporary_buffer_add.max_length / 2 {
            let _ = self
                .content
                .add_text(&self.temporary_buffer_add.buffer.clone(), self.temporary_buffer_add.position);

            self.temporary_buffer_add.clear(self.cursor_position);
        }
    }
}
