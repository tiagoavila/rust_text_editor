use crate::prelude::{
    EnumAddResult, PieceTable, Position, TemporaryBufferAddText, TemporaryBufferDeleteText,
    TextTrait,
};
use crossterm::event::KeyCode;
use std::collections::HashMap;

pub struct Editor {
    content: PieceTable,
    pub text_position: usize,
    pub temporary_add_buffer: TemporaryBufferAddText,
    pub temporary_delete_buffer: TemporaryBufferDeleteText,
    pub cursor: Position,
    right_most_column: u16,
    pub lines_map: Vec<usize>,
}

impl Editor {
    /// Creates a new Editor instance with the given initial text and temporary buffer size.
    /// Initializes the piece table, buffers, cursor position, and line map.
    pub fn new(text: String, temporary_buffer_max_length: usize) -> Self {
        let mut text_position = 0; // Start at the end of the text
        if !text.is_empty() {
            // If the text is not empty, set the cursor position to the end of the text
            text_position = text.len();
        }

        let mut editor = Self {
            content: PieceTable::new(&text.clone()),
            temporary_add_buffer: TemporaryBufferAddText::new(
                temporary_buffer_max_length,
                text_position,
            ),
            temporary_delete_buffer: TemporaryBufferDeleteText::new(temporary_buffer_max_length),
            text_position,
            cursor: Position {
                x: 0,
                y: 0,
            },
            lines_map: Vec::new(),
            right_most_column: 0,
        };

        editor.update_lines_map();

        let last_line_length = editor.lines_map.last().cloned().unwrap_or(0);
        editor.cursor = Position {
            x: last_line_length as u16,
            y: editor.lines_map.len() as u16 - 1, // Set cursor to the last line
        };

        editor
    }

    /// Adds a character at the current cursor position using the temporary add buffer.
    /// Persists the delete buffer if needed, updates buffer position, and moves the cursor.
    pub fn add_char(&mut self, c: char) {
        use crate::prelude::EnumAddResult;

        if !self.temporary_delete_buffer.is_empty() {
            self.persist_delete_buffer();
        }

        if self.temporary_add_buffer.buffer.is_empty() {
            // If the temporary buffer is empty, we can set its position to the current cursor position
            self.temporary_add_buffer
                .update_position(self.text_position);
        }

        let add_result = self.temporary_add_buffer.add_char(c);

        self.text_position += 1;
        self.cursor.move_right();
        self.set_right_most_column(self.cursor.x);

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
        } else if !self.temporary_delete_buffer.is_empty() {
            // If the delete buffer is not empty, we should not show the deleted text
            if let Some((start, end)) = self.temporary_delete_buffer.get_deletion_range() {
                content.replace_range(start..end, "");
            }
        }

        content
    }

    /// Returns the current text in the editor as a vector of lines.
    pub fn get_text_lines(&self) -> Vec<String> {
        self.get_text()
            .split("\n")
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
    }

    /// Deletes a character at the current cursor position.
    /// Handles both the temporary add buffer and the delete buffer, and updates the cursor.
    pub fn delete_char(&mut self, key: KeyCode) {
        if self.text_position > 0 {
            let deleted_position = self.text_position;

            // If the cursor is on the temporary buffer add, remove the character from it at the end
            if !self.temporary_add_buffer.buffer.is_empty()
                && self
                    .temporary_add_buffer
                    .is_cursor_on_buffer(self.text_position)
            {
                self.temporary_add_buffer.delete_char();
            } else {
                if let Ok(EnumAddResult::MustPersist) =
                    self.temporary_delete_buffer.add_char(deleted_position, key)
                {
                    // If the delete buffer is full, then delete the text range from the piece table
                    self.persist_delete_buffer();
                }
            }

            if key == KeyCode::Backspace {
                self.text_position -= 1; // Move cursor back before deleting with backspace
                self.cursor.move_left();
                self.set_right_most_column(self.cursor.x);
            }
        }
    }

    /// Deletes a word at the current cursor position.
    /// Persists the add buffer if needed and updates the cursor and buffers accordingly.
    pub fn delete_word(&mut self, key: KeyCode) {
        if !self.temporary_add_buffer.buffer.is_empty() {
            self.persist_add_buffer(true);
        }

        let delete_result =
            self.temporary_delete_buffer
                .delete_word(&self.get_text(), self.text_position, key);

        if key == KeyCode::Backspace {
            if let Some((start, _end)) = self.temporary_delete_buffer.get_deletion_range() {
                self.text_position = start; // Update cursor position to the start of the deletion range
                self.temporary_add_buffer
                    .update_position(self.text_position);
            }
        }

        if let Ok(EnumAddResult::MustPersist) = delete_result {
            self.persist_delete_buffer();
        }
    }

    /// Moves the cursor one position to the left, updating the text position and line map.
    pub fn move_cursor_left(&mut self) {
        if self.text_position > 0 {
            self.text_position -= 1;
            self.cursor.move_left();
            self.set_right_most_column(self.cursor.x);
            self.do_after_move_cursor();
        }
    }

    /// Moves the cursor one position to the right, updating the text position and line map.
    pub fn move_cursor_right(&mut self) {
        if self.text_position < self.content.total_length() {
            self.text_position += 1;
            self.cursor.move_right();
            self.set_right_most_column(self.cursor.x);
            self.do_after_move_cursor();
        }
    }

    /// Moves the cursor up by one line, adjusting the x position if necessary.
    /// Updates the text position and line map.
    pub fn move_cursor_up(&mut self) {
        self.cursor.move_up();
        self.handle_change_of_cursor_y_position();
        self.do_after_move_cursor();
        // TODO: Implement logic to move the cursor up in the content by updating the text_position value
    }

    /// Moves the cursor down by one line, adjusting the x position if necessary.
    /// Updates the text position and line map.
    pub fn move_cursor_down(&mut self) {
        self.cursor.move_down();
        self.handle_change_of_cursor_y_position();
        self.do_after_move_cursor();
        // TODO: Implement logic to move the cursor down in the content by updating the text_position value
    }

    /// Ensures the cursor's x position is valid for the current line after moving up or down.
    /// Adjusts x to the last character if it exceeds the line length.
    fn handle_change_of_cursor_y_position(&mut self) {
        let line_index = self.cursor.y as usize;
        let line_length = self.lines_map.get(line_index).cloned().unwrap_or(0);
        if line_length < self.cursor.x as usize {
            // If the cursor x position is greater than the line length, we need to adjust it
            self.cursor.x = line_length as u16; // Set to the last character of the line
        } else {
            self.cursor.x = self.right_most_column;
        }
        
        self.update_text_position_after_cursor_move();
    }

    /// Updates the text position after moving the cursor.
    /// This function recalculates the text position based on the current cursor position.
    /// It sums the lengths of all lines up to the current line and adds the x position
    /// of the cursor to get the total character count up to the cursor.
    fn update_text_position_after_cursor_move(&mut self) {
        let chars_count_up_to_previous_line: usize = self
            .lines_map
            .iter()
            .take(self.cursor.y as usize)
            .fold(0, |acc, &line_length| {
                acc + line_length + 1 // +1 for the newline character
            });
        self.text_position = chars_count_up_to_previous_line + self.cursor.x as usize;
    }

    /// Adds a new line at the current cursor position.
    /// Persists any changes, inserts a newline, updates buffers, and resets the rightmost column.
    pub fn add_new_line(&mut self) {
        self.persist_changes();

        let _ = self.content.add_text(&format!("\n"), self.text_position);
        self.cursor.move_to_new_line();
        self.text_position += 1;
        self.temporary_add_buffer
            .update_position(self.text_position);
        self.update_lines_map();
        self.set_right_most_column(0);
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
        if force_save
            || self.temporary_add_buffer.buffer.len() > self.temporary_add_buffer.max_length / 2
        {
            let _ = self.content.add_text(
                &self.temporary_add_buffer.buffer.clone(),
                self.temporary_add_buffer.position,
            );

            self.temporary_add_buffer.clear(self.text_position);
        }
    }

    /// Persists the contents of the temporary delete buffer to the piece table.
    /// Deletes the text range from the piece table and clears the delete buffer.
    fn persist_delete_buffer(&mut self) {
        if let Some((start, end)) = self.temporary_delete_buffer.get_deletion_range() {
            let _ = self.content.delete_text(start, end);
            self.temporary_delete_buffer.clear();
        }
    }

    /// Persists both the add and delete buffers to the piece table.
    /// Used to flush all temporary changes before certain operations.
    fn persist_changes(&mut self) {
        self.persist_add_buffer(true);
        self.persist_delete_buffer();
    }

    /// Called after every cursor movement.
    /// Persists any changes in the temporary buffers, updates buffer positions, and updates the line map.
    fn do_after_move_cursor(&mut self) {
        self.persist_changes();
        self.temporary_add_buffer
            .update_position(self.text_position);
        self.update_lines_map();
    }

    /// Generates a map of line numbers to their lengths based on the current text.
    /// Updates the internal lines_map field.
    fn update_lines_map(&mut self) {
        // This function updates the lines map based on the current content
        let mut lines_map: Vec<usize> = Vec::new();
        for line in self.get_text_lines().into_iter() {
            lines_map.push(line.len());
        }
        self.lines_map = lines_map;
    }

    /// Sets the rightmost column value for the cursor.
    fn set_right_most_column(&mut self, column: u16) {
        self.right_most_column = column;
    }
}
