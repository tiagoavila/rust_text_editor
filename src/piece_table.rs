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

        let mut piece_start_position: usize = 0;

        if !self.add_buffer.is_empty() {
            piece_start_position = self.add_buffer.len();
        }

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
                    length: left.len()
                };
                let last_piece = Piece {
                    buffer_type: BufferType::Original,
                    start: left.len(),
                    length: right.len()
                };

                self.pieces = vec![first_piece, new_piece, last_piece];
            } else {
                let mut content_size = 0;
                for (index, piece) in self.pieces.iter().enumerate() {
                    content_size += piece.length;
                    if content_size > position {
                        let left_piece = Piece {
                            buffer_type: piece.buffer_type.clone(),
                            start: piece.start,
                            length: content_size - piece.length,
                        };
                        let right_piece = Piece {
                            buffer_type: piece.buffer_type.clone(),
                            start: piece.start + left_piece.length,
                            length: piece.length - left_piece.length,
                        };

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_table_initialization() {
        let text: &'static str = "Hello, world!";
        let piece_table: PieceTable = PieceTable::new(text);

        // Check if the original buffer contains the given text
        assert_eq!(piece_table.original_buffer, text);

        // Check if the add buffer is empty
        assert_eq!(piece_table.add_buffer, "");

        // Check if the pieces vector has exactly one piece
        assert_eq!(piece_table.pieces.len(), 1);

        // Check the properties of the single piece
        let piece: &Piece = &piece_table.pieces[0];
        assert_eq!(piece.buffer_type, BufferType::Original);
        assert_eq!(piece.start, 0);
        assert_eq!(piece.length, text.len());
    }

    #[test]
    fn test_add_text_in_the_middle_with_empty_add_buffer() {
        let mut piece_table = PieceTable::new("Hello world");

        // Add text at position 7 (after "Hello, ")
        let result = piece_table.add_text("beautiful ", 5);

        // Ensure the operation was successful
        assert!(result.is_ok());

        // Check if the add buffer contains the added text
        assert_eq!(piece_table.add_buffer, "beautiful ");

        // Check if the pieces vector has been updated correctly
        assert_eq!(piece_table.pieces.len(), 3);

        // Verify the first piece (original buffer up to position 7)
        let first_piece = &piece_table.pieces[0];
        assert_eq!(first_piece.buffer_type, BufferType::Original);
        assert_eq!(first_piece.start, 0);
        assert_eq!(first_piece.length, 5);

        // Verify the second piece (added text)
        let second_piece = &piece_table.pieces[1];
        assert_eq!(second_piece.buffer_type, BufferType::Added);
        assert_eq!(second_piece.start, 0);
        assert_eq!(second_piece.length, 10);

        // Verify the third piece (remaining original buffer)
        let third_piece = &piece_table.pieces[2];
        assert_eq!(third_piece.buffer_type, BufferType::Original);
        assert_eq!(third_piece.start, 5);
        assert_eq!(third_piece.length, 6);
    }

    #[test]
    fn test_add_text_at_beginning() {
        let mut piece_table = PieceTable::new("world!");

        // Insert at the very beginning
        let result = piece_table.add_text("Hello, ", 0);

        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, "Hello, ");
        assert_eq!(piece_table.pieces.len(), 2);

        // First piece should be the added text
        let first_piece = &piece_table.pieces[0];
        assert_eq!(first_piece.buffer_type, BufferType::Added);
        assert_eq!(first_piece.start, 0);
        assert_eq!(first_piece.length, 7);

        // Second piece should be the original buffer
        let second_piece = &piece_table.pieces[1];
        assert_eq!(second_piece.buffer_type, BufferType::Original);
        assert_eq!(second_piece.start, 0);
        assert_eq!(second_piece.length, 6);
    }

    #[test]
    fn test_add_text_at_end() {
        let mut piece_table = PieceTable::new("Hello");

        // Insert at the very end
        let result = piece_table.add_text(", world!", 5);

        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, ", world!");
        assert_eq!(piece_table.pieces.len(), 2);

        // First piece should be the original buffer
        let first_piece = &piece_table.pieces[0];
        assert_eq!(first_piece.buffer_type, BufferType::Original);
        assert_eq!(first_piece.start, 0);
        assert_eq!(first_piece.length, 5);

        // Second piece should be the added text
        let second_piece = &piece_table.pieces[1];
        assert_eq!(second_piece.buffer_type, BufferType::Added);
        assert_eq!(second_piece.start, 0);
        assert_eq!(second_piece.length, 8);
    }

    #[test]
    fn test_multiple_insertions_various_positions() {
        let mut piece_table = PieceTable::new("Hello world");

        // 1. Insert at the end
        let result = piece_table.add_text("!", 11);
        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, "!");
        assert_eq!(piece_table.pieces.len(), 2);

        // 2. Insert at the beginning
        let result = piece_table.add_text("Say: ", 0);
        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, "!Say: ");
        assert_eq!(piece_table.pieces.len(), 3);

        // 3. Insert in the middle (after "Say: Hello", which is position 10)
        let result = piece_table.add_text(" beautiful", 10);
        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, "!Say:  beautiful");
        assert_eq!(piece_table.pieces.len(), 5);

        // Check the pieces
        // After all insertions, the pieces should represent:
        // [Say: ] [Hello ] [beautiful] [world] [!]
        let p = &piece_table.pieces;
        assert_eq!(p[0].buffer_type, BufferType::Added);    // Say: 
        assert_eq!(p[0].start, 1);
        assert_eq!(p[0].length, 5);

        assert_eq!(p[1].buffer_type, BufferType::Original); // Hello 
        assert_eq!(p[1].start, 0);
        assert_eq!(p[1].length, 5);

        assert_eq!(p[2].buffer_type, BufferType::Added);    // beautiful
        assert_eq!(p[2].start, 6);
        assert_eq!(p[2].length, 10);

        assert_eq!(p[3].buffer_type, BufferType::Original); // world
        assert_eq!(p[3].start, 5);
        assert_eq!(p[3].length, 6);

        assert_eq!(p[4].buffer_type, BufferType::Added);    // !
        assert_eq!(p[4].start, 0);
        assert_eq!(p[4].length, 1);
    }

    #[test]
    fn test_multiple_middle_insertions() {
        let mut piece_table = PieceTable::new("Hello world");

        // Insert at the end
        let result = piece_table.add_text("!", 11);
        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, "!");
        assert_eq!(piece_table.pieces.len(), 2);

        // Insert at the beginning
        let result = piece_table.add_text("Say: ", 0);
        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, "!Say: ");
        assert_eq!(piece_table.pieces.len(), 3);

        // Insert in the middle (after "Say: Hello", position 10)
        let result = piece_table.add_text(" beautiful", 10);
        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, "!Say:  beautiful");
        assert_eq!(piece_table.pieces.len(), 5);

        // Insert another in the middle (after "Say: Hello beautiful", position 20)
        let result = piece_table.add_text(" amazing", 20);
        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, "!Say:  beautiful amazing");
        assert_eq!(piece_table.pieces.len(), 6);

        // Insert yet another in the middle (after "Say: Hello beautiful amazing", position 28)
        let result = piece_table.add_text(" and cool", 28);
        assert!(result.is_ok());
        assert_eq!(piece_table.add_buffer, "!Say:  beautiful amazing and cool");
        assert_eq!(piece_table.pieces.len(), 7);

        // Check the pieces
        // The expected sequence is:
        // [Say: ] [Hello ] [beautiful] [ amazing] [ and cool] [world] [!]
        let p = &piece_table.pieces;
        assert_eq!(p[0].buffer_type, BufferType::Added);    // Say: 
        assert_eq!(p[0].start, 1);
        assert_eq!(p[0].length, 5);

        assert_eq!(p[1].buffer_type, BufferType::Original); // Hello 
        assert_eq!(p[1].start, 0);
        assert_eq!(p[1].length, 5);

        assert_eq!(p[2].buffer_type, BufferType::Added);    // beautiful
        assert_eq!(p[2].start, 6);
        assert_eq!(p[2].length, 10);

        assert_eq!(p[3].buffer_type, BufferType::Added);    // amazing
        assert_eq!(p[3].start, 16);
        assert_eq!(p[3].length, 8);

        assert_eq!(p[4].buffer_type, BufferType::Added);    // and cool
        assert_eq!(p[4].start, 24);
        assert_eq!(p[4].length, 9);

        assert_eq!(p[5].buffer_type, BufferType::Original); // world
        assert_eq!(p[5].start, 5);
        assert_eq!(p[5].length, 6);

        assert_eq!(p[6].buffer_type, BufferType::Added);    // !
        assert_eq!(p[6].start, 0);
        assert_eq!(p[6].length, 1);
    }
}
