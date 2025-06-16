use crate::prelude::EnumAddResult;

/// A buffer for temporarily holding text before persisting to the piece table.
pub struct TemporaryBufferAddText {
    pub buffer: String,
    pub max_length: usize,
    pub position: usize,
}

impl TemporaryBufferAddText {
    pub fn new(max_length: usize, cursor_position: usize) -> Self {
        Self {
            buffer: String::new(),
            max_length,
            position: cursor_position,
        }
    }

    pub fn add_char(&mut self, c: char) -> Result<EnumAddResult, ()> {
        if self.buffer.len() >= self.max_length {
            return Err(());
        }
        
        self.buffer.push(c);
        
        if self.buffer.len() == self.max_length {
            Ok(EnumAddResult::MustPersist)
        } else {
            Ok(EnumAddResult::Added)
        }
    }
    
    pub fn update_position(&mut self, new_position: usize) {
        self.position = new_position;
    }

    pub fn delete_char(&mut self) {
        if self.position > 0 {
            self.buffer.pop();
        }
    }

    pub fn clear(&mut self, cursor_position: usize) {
        self.buffer.clear();
        self.position = cursor_position;
    }
    
    pub fn is_cursor_on_buffer(&self, cursor_position: usize) -> bool {
        let end = self.position + self.buffer.len();
        cursor_position >= self.position && cursor_position < end
    }
}