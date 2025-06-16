/// A buffer for temporarily holding text before persisting to the piece table.
pub struct TemporaryBuffer {
    pub buffer: String,
    pub max_length: usize,
    pub position: usize,
}

impl TemporaryBuffer {
    pub fn new(max_length: usize, cursor_position: usize) -> Self {
        Self {
            buffer: String::new(),
            max_length,
            position: cursor_position,
        }
    }

    pub fn add_char(&mut self, c: char) -> Result<AddResult, ()> {
        if self.buffer.len() >= self.max_length {
            return Err(());
        }
        
        self.buffer.push(c);
        
        if self.buffer.len() == self.max_length {
            Ok(AddResult::MustPersist)
        } else {
            Ok(AddResult::Added)
        }
    }
    
    pub fn update_position(&mut self, new_position: usize) {
        self.position = new_position;
    }

    pub fn remove_char(&mut self) {
        if self.position > 0 {
            self.buffer.pop();
            self.position -= 1;
        }
    }

    pub fn clear(&mut self, cursor_position: usize) {
        self.buffer.clear();
        self.position = cursor_position;
    }
}

/// Result of attempting to add a character to the TemporaryBuffer.
pub enum AddResult {
    Added,
    MustPersist,
}