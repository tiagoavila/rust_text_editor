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
    length: usize
}

#[derive(Debug, Clone, PartialEq)]
enum BufferType {
    Original,
    Added
}

impl TextTrait for PieceTable {
    fn new(text: &str) -> Self {
        let original_buffer = text.to_string();
        let pieces: Vec<Piece> = vec![
            Piece {
                buffer_type: BufferType::Original,
                start: 0,
                length: original_buffer.len(),
            }
        ];

        PieceTable {
            original_buffer,
            add_buffer: String::new(),
            pieces,
        }
    }
    
    fn add_text(&mut self, text: &str, position: usize) -> Result<(), String> {
        todo!()
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
    fn test_add_text() {
        let mut piece_table = PieceTable::new("Hello, world!");

        // Add text at position 7 (after "Hello, ")
        let result = piece_table.add_text("Rust", 7);

        // Ensure the operation was successful
        assert!(result.is_ok());

        // Check if the add buffer contains the added text
        assert_eq!(piece_table.add_buffer, "Rust");

        // Check if the pieces vector has been updated correctly
        assert_eq!(piece_table.pieces.len(), 3);

        // Verify the first piece (original buffer up to position 7)
        let first_piece = &piece_table.pieces[0];
        assert_eq!(first_piece.buffer_type, BufferType::Original);
        assert_eq!(first_piece.start, 0);
        assert_eq!(first_piece.length, 7);

        // Verify the second piece (added text)
        let second_piece = &piece_table.pieces[1];
        assert_eq!(second_piece.buffer_type, BufferType::Added);
        assert_eq!(second_piece.start, 0);
        assert_eq!(second_piece.length, 4);

        // Verify the third piece (remaining original buffer)
        let third_piece = &piece_table.pieces[2];
        assert_eq!(third_piece.buffer_type, BufferType::Original);
        assert_eq!(third_piece.start, 7);
        assert_eq!(third_piece.length, 6);
    }
}