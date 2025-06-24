use crate::prelude::EnumAddResult;
use crossterm::event::KeyCode; // Add this line or adjust the path to where KeyCode is defined

pub struct TemporaryBufferDeleteText {
    max_length: usize,
    start: Option<usize>,
    end: Option<usize>,
}

impl TemporaryBufferDeleteText {
    pub fn new(max_length: usize) -> Self {
        Self {
            max_length,
            start: None,
            end: None,
        }
    }

    pub fn add_char(&mut self, position: usize, key: KeyCode) -> Result<EnumAddResult, ()> {
        if self.start.is_none() {
            if key != KeyCode::Backspace && key != KeyCode::Delete {
                return Err(());
            }

            if key == KeyCode::Backspace {
                if position == 0 {
                    return Ok(EnumAddResult::NoChange);
                }

                self.start = Some(position - 1);
                self.end = Some(position);
            } else {
                self.start = Some(position);
                self.end = Some(position + 1);
            }
        
            return Ok(EnumAddResult::Added);
        }
        
        let start = self.start.unwrap();
        let end = self.end.unwrap();
        
        if key == KeyCode::Delete {
            // If the delete key is pressed at the end of the current range, extend the end
            self.end = Some(end + 1);
        } else if key == KeyCode::Backspace {
            // If the backspace key is pressed at the start of the current range, extend the start
            self.start = Some(start - 1);
        } else {
            // If neither key is pressed at the correct position, return no change
            return Ok(EnumAddResult::NoChange);
        }
            
        if self.end.unwrap() - self.start.unwrap() == self.max_length {
            Ok(EnumAddResult::MustPersist)
        } else {
            Ok(EnumAddResult::Added)
        }
    }

    pub fn get_deletion_range(&self) -> Option<(usize, usize)> {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            Some((start, end))
        } else {
            None
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    pub fn clear(&mut self) {
        self.start = None;
        self.end = None;
    }
}
