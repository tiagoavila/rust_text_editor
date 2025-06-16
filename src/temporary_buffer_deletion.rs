use crate::prelude::EnumAddResult;

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

    pub fn add_char(&mut self, position: usize) -> Result<EnumAddResult, ()> {
        if self.start.is_none() {
            self.start = Some(position);
            self.end = Some(position);
            return Ok(EnumAddResult::Added);
        }
        
        let start = self.start.unwrap();
        let end = self.end.unwrap();

        if start > position || end < position {
            return Err(());
        }

        if position < start {
            self.start = Some(position);
        } else if position > end {
            self.end = Some(position);
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

    pub fn clear(&mut self) {
        self.start = None;
        self.end = None;
    }
}
