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

    fn delete_text(&mut self, position: usize, length: usize) -> Result<(), String> {
        let mut content_size: usize = 0;
        let start_piece: Piece;
        let end_piece: Piece;

        for (index, piece) in self.pieces.iter().enumerate() {
            content_size += piece.length;

            let end_index = position + length;

            // Delete text in the middle of a single piece, which results in 2 new pieces
            if content_size > position && end_index < content_size {
                // Calculate the start of the current piece in the logical text
                let piece_logical_start = content_size - piece.length;
                let left_piece = Piece {
                    buffer_type: piece.buffer_type.clone(),
                    start: piece_logical_start,
                    length: position - piece_logical_start
                };

                let right_piece = Piece {
                    buffer_type: piece.buffer_type.clone(),
                    start: piece_logical_start + left_piece.length + length,
                    length: piece.length - (left_piece.length + length)
                };
                
                self.pieces[index] = left_piece;
                self.pieces.insert(index + 1, right_piece);
                break;
            }

            //handle deletion of text from multiple pieces
        }

        Ok(())
    }
}

impl PieceTable {
    pub(crate) fn get_text_from_buffer(buffer: &str, result: &mut String, piece: &Piece) {
        result.push_str(&buffer[piece.start..(piece.start + piece.length)].to_string())
    }
}

#[cfg(test)]
mod tests;
