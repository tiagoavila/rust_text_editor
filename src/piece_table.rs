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

    fn delete_text(&mut self, start: usize, end: usize) -> Result<(), String> {
        let mut content_size: usize = 0;
        let mut start_piece_index: usize = 0;
        let mut start_piece_logical_start = 0;
        let mut end_piece_index: usize = 0;
        let mut end_piece_logical_start: usize = 0;

        // find indexes of start and end pieces
        for (index, piece) in self.pieces.iter().enumerate() {
            content_size += piece.length;

            if content_size > start {
                start_piece_index = index;
                start_piece_logical_start = content_size - piece.length;
            }

            if content_size > end {
                end_piece_index = index;
                break;
            }
        }

        // Calculate the start of the current piece in the logical text
        let piece: Piece = self.pieces[start_piece_index].clone();
        let piece_before_delete: Piece = Piece {
            buffer_type: piece.buffer_type.clone(),
            start: piece.start,
            length: start - start_piece_logical_start,
        };
        self.pieces[start_piece_index] = piece_before_delete.clone();

        // TODO use extend_from_slice and push as code generated here https://claude.ai/chat/24192bee-dd0f-4f81-9d33-6a5843f99b41

        if start_piece_index == end_piece_index && end < content_size - 1 {
            let piece_after_delete: Piece = Piece {
                buffer_type: piece.buffer_type.clone(),
                start: piece_before_delete.start + piece_before_delete.length + get_delete_size(start, end),
                length: piece.length - piece_before_delete.length - get_delete_size(start, end),
            };
            self.pieces.insert(start_piece_index + 1, piece_after_delete);
        }

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
}

#[cfg(test)]
mod tests;
