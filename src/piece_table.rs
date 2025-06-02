use crate::prelude::TextTrait;
//https://docs.rs/crossterm/latest/crossterm/

#[derive(Debug)]
pub struct PieceTable {
    original_buffer: String,
    add_buffer: String,
    pieces: Vec<Piece>,
}

#[derive(Debug, Clone)]
pub struct Piece {
    buffer_type: BufferType,
    start: usize,
    length: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum BufferType {
    Original,
    Added,
}

impl TextTrait for PieceTable {
    fn new(text: &str) -> Self {
        let original_buffer = text.to_string();
        let pieces: Vec<Piece> = vec![Piece {
            buffer_type: BufferType::Original,
            start: 0,
            length: original_buffer.len(),
        }];

        PieceTable {
            original_buffer,
            add_buffer: String::new(),
            pieces,
        }
    }

    fn add_text(&mut self, text: &str, position: usize) -> Result<(), String> {
        if text.is_empty() {
            return Ok(());
        }

        let piece_start_position = self.add_buffer.len();
        self.add_buffer.push_str(text);

        let new_piece = Piece {
            buffer_type: BufferType::Added,
            start: piece_start_position,
            length: text.len(),
        };

        if position == 0 {
            self.pieces.insert(0, new_piece);
        } else if position == self.original_buffer.len() {
            self.pieces.push(new_piece);
        } else {
            if self.pieces.len() == 1 {
                let (left, right) = self.original_buffer.split_at(position);
                let first_piece = Piece {
                    buffer_type: BufferType::Original,
                    start: 0,
                    length: left.len(),
                };
                let last_piece = Piece {
                    buffer_type: BufferType::Original,
                    start: left.len(),
                    length: right.len(),
                };

                self.pieces = vec![first_piece, new_piece, last_piece];
            } else {
                let mut content_size = 0;
                for (index, piece) in self.pieces.iter().enumerate() {
                    content_size += piece.length;
                    if content_size > position {
                        // Calculate the start of the current piece in the logical text
                        let piece_logical_start = content_size - piece.length;
                        // The left piece should cover from the start of this piece up to the insertion point
                        let left_piece_length = position - piece_logical_start;

                        let left_piece = Piece {
                            buffer_type: piece.buffer_type.clone(),
                            start: piece.start,
                            length: left_piece_length,
                        };
                        let right_piece = Piece {
                            buffer_type: piece.buffer_type.clone(),
                            start: piece.start + left_piece_length,
                            length: piece.length - left_piece_length,
                        };

                        // Replace the current piece with the left piece, then insert the new and right pieces
                        self.pieces[index] = left_piece;
                        self.pieces.insert(index + 1, new_piece);
                        self.pieces.insert(index + 2, right_piece);
                        break;
                    } else if content_size == position {
                        self.pieces.insert(index + 1, new_piece);
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn get_text(&self) -> String {
        let mut result = String::new();

        for piece in self.pieces.iter() {
            match piece.buffer_type {
                BufferType::Original => {
                    PieceTable::get_text_from_buffer(&self.original_buffer, &mut result, piece);
                }
                BufferType::Added => {
                    PieceTable::get_text_from_buffer(&self.add_buffer, &mut result, piece);
                }
            };
        }

        result
    }

    /// Deletes a range of text from the piece table using start and end indices.
    /// 
    /// This operation efficiently handles deletions by modifying the piece sequence
    /// rather than actually moving text data. It can handle:
    /// - Deletions within a single piece (splits the piece)
    /// - Deletions spanning multiple pieces (removes/modifies affected pieces)
    /// - Edge cases like deletions at text boundaries
    /// 
    /// # Arguments
    /// * `start` - The starting index of the deletion (0-based, inclusive)
    /// * `end` - The ending index of the deletion (0-based, exclusive)
    /// 
    /// # Returns
    /// * `Ok(())` if deletion was successful
    /// * `Err(String)` with error message if parameters are invalid
    /// 
    /// # Note
    /// This uses the standard range convention where `start` is inclusive and `end` is exclusive,
    /// meaning the deletion affects characters from `start` to `end - 1`.
    fn delete_text(&mut self, start: usize, end: usize) -> Result<(), String> {
        let total_len = self.total_length();
        
        // Validate deletion parameters
        if start > total_len {
            return Err(format!("Start index {} is beyond text length {}", start, total_len));
        }
        
        if end > total_len {
            return Err(format!("End index {} is beyond text length {}", end, total_len));
        }
        
        if start > end {
            return Err(format!("Start index {} cannot be greater than end index {}", start, end));
        }
        
        // Handle trivial case - nothing to delete (empty range)
        if start == end {
            return Ok(());
        }

        // Find pieces affected by the deletion by walking through the piece sequence
        let mut current_pos = 0;          // Current position in the logical text
        let mut start_piece_idx = None;   // Index of piece containing deletion start
        let mut end_piece_idx = None;     // Index of piece containing deletion end
        let mut start_offset = 0;         // Offset within start piece where deletion begins
        let mut end_offset = 0;           // Offset within end piece where deletion ends

        for (i, piece) in self.pieces.iter().enumerate() {
            let piece_end = current_pos + piece.length;
            
            // Find the piece containing the start position
            if start_piece_idx.is_none() && start >= current_pos && start < piece_end {
                start_piece_idx = Some(i);
                start_offset = start - current_pos;
            }
            
            // Find the piece containing the end position
            // Note: end can equal piece_end (deletion ends at piece boundary)
            if end > current_pos && end <= piece_end {
                end_piece_idx = Some(i);
                end_offset = end - current_pos;
                break;
            }
            
            current_pos = piece_end;
        }

        let start_idx = start_piece_idx.ok_or("Could not find start piece")?;
        let end_idx = end_piece_idx.unwrap_or(self.pieces.len() - 1);

        // Build new piece sequence without the deleted content
        let mut new_pieces = Vec::new();

        // 1. Keep all pieces that come before the deletion range
        new_pieces.extend_from_slice(&self.pieces[..start_idx]);

        // 2. Handle the start piece - keep the part before the deletion starts
        if start_offset > 0 {
            let start_piece = &self.pieces[start_idx];
            new_pieces.push(Piece {
                buffer_type: start_piece.buffer_type.clone(),
                start: start_piece.start,
                length: start_offset,  // Only keep text before deletion
            });
        }

        // 3. Handle the end piece - keep the part after the deletion ends
        if end_idx < self.pieces.len() {
            let end_piece = &self.pieces[end_idx];
            if end_offset < end_piece.length {
                new_pieces.push(Piece {
                    buffer_type: end_piece.buffer_type.clone(),
                    start: end_piece.start + end_offset,  // Skip the deleted part
                    length: end_piece.length - end_offset, // Remaining length
                });
            }
        }

        // 4. Keep all pieces that come after the deletion range
        if end_idx + 1 < self.pieces.len() {
            new_pieces.extend_from_slice(&self.pieces[end_idx + 1..]);
        }

        // Replace the old piece sequence with the new one
        self.pieces = new_pieces;
        Ok(())
    }

}

/// Calculates the number of characters to delete given a start and end position (inclusive).
///
/// # Arguments
///
/// * `start` - The starting index of the deletion (inclusive).
/// * `end` - The ending index of the deletion (inclusive).
///
/// # Returns
///
/// The number of characters to delete.
///
/// # Example
///
/// ```
/// let size = get_delete_size(3, 5);
/// assert_eq!(size, 3); // Deletes positions 3, 4, and 5
/// ```
fn get_delete_size(start: usize, end: usize) -> usize {
    end - start + 1
}

impl PieceTable {
    pub(crate) fn get_text_from_buffer(buffer: &str, result: &mut String, piece: &Piece) {
        result.push_str(&buffer[piece.start..(piece.start + piece.length)].to_string())
    }

    /// Calculates the total length of text represented by all pieces
    fn total_length(&self) -> usize {
        self.pieces.iter().map(|p| p.length).sum()
    }
}

#[cfg(test)]
mod tests;
