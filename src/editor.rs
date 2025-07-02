use crossterm::event::KeyCode;

use crate::prelude::{EnumAddResult, PieceTable, Position, TemporaryBufferAddText, TemporaryBufferDeleteText, TextTrait};

pub struct Editor {
    content: PieceTable,
    pub text_position: usize,
    pub temporary_add_buffer: TemporaryBufferAddText,
    pub temporary_delete_buffer: TemporaryBufferDeleteText,
    pub cursor: Position
}

impl Editor {
    pub fn new(text: String, temporary_buffer_max_length: usize) -> Self {
        let mut cursor_position = 0; // Start at the end of the text
        if !text.is_empty() {
            // If the text is not empty, set the cursor position to the end of the text
            cursor_position = text.len() - 1;
        }

        Self {
            content: PieceTable::new(&text.clone()),
            temporary_add_buffer: TemporaryBufferAddText::new(temporary_buffer_max_length, cursor_position),
            temporary_delete_buffer: TemporaryBufferDeleteText::new(temporary_buffer_max_length),
            text_position: cursor_position,
            cursor: Position { x: cursor_position as u16, y: 0 }
        }
    }

    pub fn add_char(&mut self, c: char) {
        use crate::prelude::EnumAddResult;
        
        if !self.temporary_delete_buffer.is_empty() {
            self.persist_delete_buffer();
        }
        
        if self.temporary_add_buffer.buffer.is_empty() {
            // If the temporary buffer is empty, we can set its position to the current cursor position
            self.temporary_add_buffer.update_position(self.text_position);
        }

        let add_result = self.temporary_add_buffer.add_char(c);

        self.text_position += 1;
        self.cursor.move_right();

        // Persist the buffer if AddResult::MustPersist is returned
        if let Ok(EnumAddResult::MustPersist) = add_result {
            self.persist_add_buffer(true);
        }
    }
    
    /// Returns the current text in the editor, including any temporary buffers.
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

    /// Returns the current text in the editor as a vector of lines.
    pub fn get_text_lines(&self) -> Vec<String> {
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

        content.split("\n").map(|line| line.to_string()).collect::<Vec<String>>()
    }

    pub fn delete_char(&mut self, key: KeyCode) {
        if self.text_position > 0 {
            let deleted_position = self.text_position;

            // If the cursor is on the temporary buffer add, remove the character from it at the end
            if !self.temporary_add_buffer.buffer.is_empty() && self.temporary_add_buffer.is_cursor_on_buffer(self.text_position) {
                self.temporary_add_buffer.delete_char();
            } else {
                if let Ok(EnumAddResult::MustPersist) = self.temporary_delete_buffer.add_char(deleted_position, key) {
                    // If the delete buffer is full, then delete the text range from the piece table
                    self.persist_delete_buffer();
                }
            }

            if key == KeyCode::Backspace {
                self.text_position -= 1; // Move cursor back before deleting with backspace
                self.cursor.move_left();
            }
        }
    }
    
    pub fn delete_word(&mut self, key: KeyCode) {
        if !self.temporary_add_buffer.buffer.is_empty() {
            self.persist_add_buffer(true);
        }
        
        let delete_result = self.temporary_delete_buffer.delete_word(&self.get_text(), self.text_position, key);

        if key == KeyCode::Backspace {
            if let Some((start, _end)) = self.temporary_delete_buffer.get_deletion_range() {
                self.text_position = start; // Update cursor position to the start of the deletion range
                self.temporary_add_buffer.update_position(self.text_position);
            }
        }

        if let Ok(EnumAddResult::MustPersist) = delete_result {
            self.persist_delete_buffer();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.text_position > 0 {
            self.text_position -= 1;
            self.cursor.move_left();
            self.do_after_move_cursor(); 
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.text_position < self.content.total_length() {
            self.text_position += 1;
            self.cursor.move_right();
            self.do_after_move_cursor();
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
        self.persist_changes();

        let _ = self.content.add_text(&format!("\n"), self.text_position);
        self.cursor.x = 0;
        self.cursor.y += 1;
        self.text_position += 1;
        self.temporary_add_buffer.update_position(self.text_position);
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
    pub fn persist_add_buffer(&mut self, force_save: bool) {
        if self.temporary_add_buffer.buffer.is_empty() {
            // If the buffer is empty, we can skip persisting
            return;
        }

        // If the buffer is not empty, we need to persist its content to the piece table
        if force_save || self.temporary_add_buffer.buffer.len() > self.temporary_add_buffer.max_length / 2 {
            let _ = self
                .content
                .add_text(&self.temporary_add_buffer.buffer.clone(), self.temporary_add_buffer.position);

            self.temporary_add_buffer.clear(self.text_position);
        }
    }
    
    /// Persists the contents of the temporary delete buffer to the piece table.
    fn persist_delete_buffer(&mut self) {
        if let Some((start, end)) = self.temporary_delete_buffer.get_deletion_range() {
            let _ = self.content.delete_text(start, end);
            self.temporary_delete_buffer.clear();
        }
    }
    
    fn persist_changes(&mut self) {
        self.persist_add_buffer(true);
        self.persist_delete_buffer();
    }
    
    fn do_after_move_cursor(&mut self) {
        // This function can be used to perform any additional actions after moving the cursor
        // For example, updating the temporary buffer position or refreshing the screen
        self.persist_changes();
        self.temporary_add_buffer.update_position(self.text_position);
    }
    
}
