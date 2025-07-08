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
    /// Creates a new `PieceTable` from the given text.
    ///
    /// # Arguments
    /// * `text` - The initial text to populate the piece table.
    ///
    /// # Returns
    /// A new `PieceTable` instance containing the provided text as the original buffer.
    ///
    /// # Example
    /// ```
    /// let pt = PieceTable::new("hello");
    /// assert_eq!(pt.get_text(), "hello");
    /// ```
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

    /// Inserts new text at the specified position in the piece table.
    ///
    /// This operation efficiently adds text by updating the piece sequence,
    /// without moving existing text data. Handles insertions at any position,
    /// including splitting existing pieces as needed.
    ///
    /// # Arguments
    /// * `text` - The text to insert.
    /// * `position` - The position (0-based) at which to insert the text.
    ///
    /// # Returns
    /// * `Ok(())` if insertion was successful.
    /// * `Err(String)` with an error message if the position is invalid.
    ///
    /// # Example
    /// ```
    /// let mut pt = PieceTable::new("abc");
    /// pt.add_text("X", 1).unwrap();
    /// assert_eq!(pt.get_text(), "aXbc");
    /// ```
    fn add_text(&mut self, text: &str, position: usize) -> Result<(), String> {
        if text.is_empty() {
            return Ok(());
        }

        let total_len = self.total_length();
        if position > total_len {
            return Err(format!(
                "Position {} is beyond text length {}",
                position, total_len
            ));
        }

        // Add the new text to the add buffer and create a piece for it
        let new_piece_start_position = self.add_buffer.len();
        self.add_buffer.push_str(text);

        // Handle insertion into empty document
        if position == 0 && self.pieces.is_empty() {
            self.pieces.push(Piece {
                buffer_type: BufferType::Added,
                start: new_piece_start_position,
                length: text.len(),
            });
            return Ok(());
        }

        // Find where to insert the new piece
        let mut current_pos = 0;
        let mut insert_idx = self.pieces.len(); // Default to end if not found
        let mut split_offset = 0;

        for (index, piece) in self.pieces.iter().enumerate() {
            // Find the piece where the insertion should happen
            if position <= current_pos + piece.length {
                insert_idx = index;
                split_offset = position - current_pos;
                break;
            }
            current_pos += piece.length;
        }

        if insert_idx == self.pieces.len() {
            // Insert at the very end - just append the new piece
            self.pieces.push(Piece {
                buffer_type: BufferType::Added,
                start: new_piece_start_position,
                length: text.len(),
            });
        } else {
            // Insert in the middle - need to split an existing piece
            let piece = self.pieces[insert_idx].clone();
            self.pieces.remove(insert_idx);

            // Insert left part of the split piece (if any)
            if split_offset > 0 {
                self.pieces.insert(
                    insert_idx,
                    Piece {
                        buffer_type: piece.buffer_type.clone(),
                        start: piece.start,
                        length: split_offset,
                    },
                );
                insert_idx += 1;
            }

            // Insert the new text piece
            self.pieces.insert(
                insert_idx,
                Piece {
                    buffer_type: BufferType::Added,
                    start: new_piece_start_position,
                    length: text.len(),
                },
            );
            insert_idx += 1;

            // Insert right part of the split piece (if any)
            if split_offset < piece.length {
                self.pieces.insert(
                    insert_idx,
                    Piece {
                        buffer_type: piece.buffer_type,
                        start: piece.start + split_offset,
                        length: piece.length - split_offset,
                    },
                );
            }
        }

        Ok(())
    }

    /// Returns the full text represented by the piece table as a `String`.
    ///
    /// This method reconstructs the current state of the text by iterating
    /// through all pieces and concatenating their corresponding slices from
    /// the original and added buffers.
    ///
    /// # Returns
    /// A `String` containing the current text.
    ///
    /// # Example
    /// ```
    /// let mut pt = PieceTable::new("abc");
    /// pt.add_text("X", 1).unwrap();
    /// assert_eq!(pt.get_text(), "aXbc");
    /// ```
    fn get_text(&self) -> String {
        let mut result = String::new();

        // Iterate over each piece and append its text to the result
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
    ///
    /// # Example
    /// ```
    /// let mut pt = PieceTable::new("abcdef");
    /// pt.delete_text(2, 4).unwrap();
    /// assert_eq!(pt.get_text(), "abef");
    /// ```
    fn delete_text(&mut self, start: usize, end: usize) -> Result<(), String> {
        let total_len = self.total_length();

        // Validate deletion parameters
        if start > total_len {
            return Err(format!(
                "Start index {} is beyond text length {}",
                start, total_len
            ));
        }

        if end > total_len {
            return Err(format!(
                "End index {} is beyond text length {}",
                end, total_len
            ));
        }

        if start > end {
            return Err(format!(
                "Start index {} cannot be greater than end index {}",
                start, end
            ));
        }

        // Handle trivial case - nothing to delete (empty range)
        if start == end {
            return Ok(());
        }

        // Find pieces affected by the deletion by walking through the piece sequence
        let mut current_pos = 0; // Current position in the logical text
        let mut start_piece_idx = None; // Index of piece containing deletion start
        let mut end_piece_idx = None; // Index of piece containing deletion end
        let mut start_offset = 0; // Offset within start piece where deletion begins
        let mut end_offset = 0; // Offset within end piece where deletion ends

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
                length: start_offset, // Only keep text before deletion
            });
        }

        // 3. Handle the end piece - keep the part after the deletion ends
        if end_idx < self.pieces.len() {
            let end_piece = &self.pieces[end_idx];
            if end_offset < end_piece.length {
                new_pieces.push(Piece {
                    buffer_type: end_piece.buffer_type.clone(),
                    start: end_piece.start + end_offset, // Skip the deleted part
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
    pub fn total_length(&self) -> usize {
        self.pieces.iter().map(|p| p.length).sum()
    }
}


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
    assert_eq!(p[0].buffer_type, BufferType::Added); // Say:
    assert_eq!(p[0].start, 1);
    assert_eq!(p[0].length, 5);

    assert_eq!(p[1].buffer_type, BufferType::Original); // Hello
    assert_eq!(p[1].start, 0);
    assert_eq!(p[1].length, 5);

    assert_eq!(p[2].buffer_type, BufferType::Added); // beautiful
    assert_eq!(p[2].start, 6);
    assert_eq!(p[2].length, 10);

    assert_eq!(p[3].buffer_type, BufferType::Original); // world
    assert_eq!(p[3].start, 5);
    assert_eq!(p[3].length, 6);

    assert_eq!(p[4].buffer_type, BufferType::Added); // !
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
    assert_eq!(p[0].buffer_type, BufferType::Added); // Say:
    assert_eq!(p[0].start, 1);
    assert_eq!(p[0].length, 5);

    assert_eq!(p[1].buffer_type, BufferType::Original); // Hello
    assert_eq!(p[1].start, 0);
    assert_eq!(p[1].length, 5);

    assert_eq!(p[2].buffer_type, BufferType::Added); // beautiful
    assert_eq!(p[2].start, 6);
    assert_eq!(p[2].length, 10);

    assert_eq!(p[3].buffer_type, BufferType::Added); // amazing
    assert_eq!(p[3].start, 16);
    assert_eq!(p[3].length, 8);

    assert_eq!(p[4].buffer_type, BufferType::Added); // and cool
    assert_eq!(p[4].start, 24);
    assert_eq!(p[4].length, 9);

    assert_eq!(p[5].buffer_type, BufferType::Original); // world
    assert_eq!(p[5].start, 5);
    assert_eq!(p[5].length, 6);

    assert_eq!(p[6].buffer_type, BufferType::Added); // !
    assert_eq!(p[6].start, 0);
    assert_eq!(p[6].length, 1);
}

#[test]
fn test_three_inserts_always_splitting_pieces() {
    // Create a piece table with the alphabet as content
    let mut piece_table = PieceTable::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

    // Initially, we should have a single piece for the original content
    assert_eq!(piece_table.pieces.len(), 1);
    assert_eq!(piece_table.pieces[0].buffer_type, BufferType::Original);
    assert_eq!(piece_table.pieces[0].start, 0);
    assert_eq!(piece_table.pieces[0].length, 26); // Alphabet has 26 letters

    // FIRST INSERT: Split the original piece by inserting "123" after "C" (at position 3)
    piece_table.add_text("123", 3).unwrap();

    // After the first insert, we should have 3 pieces:
    // 1. "ABC" (original, 0-3)
    // 2. "123" (added, 0-3)
    // 3. "DEFGHIJKLMNOPQRSTUVWXYZ" (original, 3-26)
    assert_eq!(piece_table.pieces.len(), 3);

    // Verify first piece (original buffer, contains "ABC")
    assert_eq!(piece_table.pieces[0].buffer_type, BufferType::Original);
    assert_eq!(piece_table.pieces[0].start, 0);
    assert_eq!(piece_table.pieces[0].length, 3);

    // Verify second piece (added buffer, contains "123")
    assert_eq!(piece_table.pieces[1].buffer_type, BufferType::Added);
    assert_eq!(piece_table.pieces[1].start, 0);
    assert_eq!(piece_table.pieces[1].length, 3);

    // Verify third piece (original buffer, contains "DEFGHIJKLMNOPQRSTUVWXYZ")
    assert_eq!(piece_table.pieces[2].buffer_type, BufferType::Original);
    assert_eq!(piece_table.pieces[2].start, 3);
    assert_eq!(piece_table.pieces[2].length, 23);

    // The add_buffer should now contain "123"
    assert_eq!(piece_table.add_buffer, "123");

    // SECOND INSERT: Split the third piece by inserting "456" after "F"
    // Logical content is now "ABC123DEFGHIJKLMNOPQRSTUVWXYZ"
    // Position of "F" is: 3 (ABC) + 3 (123) + 3 (DEF) = 9
    piece_table.add_text("456", 9).unwrap();

    // After the second insert, we should have 5 pieces:
    // 1. "ABC" (original, 0-3)
    // 2. "123" (added, 0-3)
    // 3. "DEF" (original, 3-6)
    // 4. "456" (added, 3-6)
    // 5. "GHIJKLMNOPQRSTUVWXYZ" (original, 6-26)
    assert_eq!(piece_table.pieces.len(), 5);

    // Check the third piece (original buffer, contains "DEF")
    assert_eq!(piece_table.pieces[2].buffer_type, BufferType::Original);
    assert_eq!(piece_table.pieces[2].start, 3);
    assert_eq!(piece_table.pieces[2].length, 3);

    // Check the fourth piece (added buffer, contains "456")
    assert_eq!(piece_table.pieces[3].buffer_type, BufferType::Added);
    assert_eq!(piece_table.pieces[3].start, 3);
    assert_eq!(piece_table.pieces[3].length, 3);

    // Check the fifth piece (original buffer, contains "GHIJKLMNOPQRSTUVWXYZ")
    assert_eq!(piece_table.pieces[4].buffer_type, BufferType::Original);
    assert_eq!(piece_table.pieces[4].start, 6);
    assert_eq!(piece_table.pieces[4].length, 20);

    // The add_buffer should now contain "123456"
    assert_eq!(piece_table.add_buffer, "123456");

    // THIRD INSERT: Split the fifth piece by inserting "789" after "J"
    // Logical content is now "ABC123DEF456GHIJ789KLMNOPQRSTUVWXYZ"
    // Position of "J" is: 3 (ABC) + 3 (123) + 3 (DEF) + 3 (456) + 4 (GHIJ) = 16
    piece_table.add_text("789", 16).unwrap();

    // After the third insert, we should have 7 pieces:
    // 1. "ABC" (original, 0-3)
    // 2. "123" (added, 0-3)
    // 3. "DEF" (original, 3-6)
    // 4. "456" (added, 3-6)
    // 5. "GHIJ" (original, 6-10)
    // 6. "789" (added, 6-9)
    // 7. "KLMNOPQRSTUVWXYZ" (original, 10-26)
    assert_eq!(piece_table.pieces.len(), 7);

    // Check the fifth piece (original buffer, contains "GHIJ")
    assert_eq!(piece_table.pieces[4].buffer_type, BufferType::Original);
    assert_eq!(piece_table.pieces[4].start, 6);
    assert_eq!(piece_table.pieces[4].length, 4);

    // Check the sixth piece (added buffer, contains "789")
    assert_eq!(piece_table.pieces[5].buffer_type, BufferType::Added);
    assert_eq!(piece_table.pieces[5].start, 6);
    assert_eq!(piece_table.pieces[5].length, 3);

    // Check the seventh piece (original buffer, contains "KLMNOPQRSTUVWXYZ")
    assert_eq!(piece_table.pieces[6].buffer_type, BufferType::Original);
    assert_eq!(piece_table.pieces[6].start, 10);
    assert_eq!(piece_table.pieces[6].length, 16);

    // The add_buffer should now contain "123456789"
    assert_eq!(piece_table.add_buffer, "123456789");

    // The final logical content should be "ABC123DEF456GHIJ789KLMNOPQRSTUVWXYZ"
    // But we don't need to verify that explicitly since we've checked all the pieces
}

#[test]
fn test_get_text() {
    let mut piece_table = PieceTable::new("Hello world");

    // Insert at the end
    piece_table.add_text("!", 11).unwrap();
    // Insert at the beginning
    piece_table.add_text("Say: ", 0).unwrap();
    // Insert in the middle (after "Say: Hello", position 10)
    piece_table.add_text(" beautiful", 10).unwrap();

    // The expected logical text is: "Say: Hello beautiful world!"
    let result = piece_table.get_text();
    assert_eq!(result, "Say: Hello beautiful world!");
}

#[test]
fn test_get_text_with_alphabet_and_inserts() {
    let mut piece_table = PieceTable::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

    // Insert "123" after "C" (at position 3)
    piece_table.add_text("123", 3).unwrap();
    // Insert "456" after "F" (position 9: 3 for "ABC", 3 for "123", 3 for "DEF")
    piece_table.add_text("456", 9).unwrap();
    // Insert "789" after "J" (position 16: 3+3+3+3+4)
    piece_table.add_text("789", 16).unwrap();

    // The expected logical text is: "ABC123DEF456GHIJ789KLMNOPQRSTUVWXYZ"
    let result = piece_table.get_text();
    assert_eq!(result, "ABC123DEF456GHIJ789KLMNOPQRSTUVWXYZ");
}

#[test]
fn test_delete_single_piece() {
    // Test deletion within a single piece - should split the piece

    let mut piece_table = PieceTable::new("ABCXXXXDEF");

    // Delete the X's (positions 3 to 6, length 4)
    let result = piece_table.delete_text(3, 6 + 1); // end index + 1 for exclusive

    assert!(result.is_ok());

    // The expected logical text is: "ABCDEF"
    let text = piece_table.get_text();
    assert_eq!(text, "ABCDEF");

    // Same test but now with a piece table that has an added piece
    let mut piece_table = PieceTable::new("DEFXXXXGHI");
    piece_table.add_text("ABC", 0).unwrap();
    let text = piece_table.get_text();
    assert_eq!(text, "ABCDEFXXXXGHI");

    // Delete the X's (positions 6 to 9, length 4)
    let result = piece_table.delete_text(6, 9 + 1); // end index + 1 for exclusive

    assert!(result.is_ok());


    // The expected logical text is: "ABCDEF"
    let text = piece_table.get_text();
    assert_eq!(text, "ABCDEFGHI");
}

#[test]
fn test_delete_text_to_the_end_of_a_piece() {
    // Test deletion from the end of text
    // let mut piece_table = PieceTable::new("ABCDEFXXXX");

    // // Delete the X's (positions 6 to 9, length 4)
    // let result = piece_table.delete_text(6, 9 + 1); // end index + 1 for exclusive

    // assert!(result.is_ok());

    // // The expected logical text is: "ABCDEF"
    // let text = piece_table.get_text();
    // assert_eq!(text, "ABCDEF");

    // Same test but now with a piece table that has an added piece
    let mut piece_table = PieceTable::new("DEFGHIXXXX");
    piece_table.add_text("ABC", 0).unwrap();
    let text = piece_table.get_text();
    assert_eq!(text, "ABCDEFGHIXXXX");

    // Delete the X's (positions 9 to 12, length 4)
    let result = piece_table.delete_text(9, 12 + 1); // end index + 1 for exclusive

    assert!(result.is_ok());

    // The expected logical text is: "ABCDEF"
    let text = piece_table.get_text();
    assert_eq!(text, "ABCDEFGHI");
}

#[test]
fn test_delete_text_at_start_of_a_piece() {
    // Test deletion from the start of text
    let mut piece_table = PieceTable::new("XXXXABCDEF");

    // Delete the X's (positions 0 to 3, length 4)
    let result = piece_table.delete_text(0, 3 + 1); // end index + 1 for exclusive

    assert!(result.is_ok());

    // The expected logical text is: "ABCDEF"
    let text = piece_table.get_text();
    assert_eq!(text, "ABCDEF");

    // Same test but now with a piece table that has an added piece
    let mut piece_table = PieceTable::new("XXXXDEFGHI");
    piece_table.add_text("ABC", 0).unwrap();
    let text = piece_table.get_text();
    assert_eq!(text, "ABCXXXXDEFGHI");

    // Delete the X's (positions 0 to 3, length 4)
    let result = piece_table.delete_text(3, 6 + 1); // end index + 1 for exclusive

    assert!(result.is_ok());

    // The expected logical text is: "ABCDEFGHI"
    let text = piece_table.get_text();
    assert_eq!(text, "ABCDEFGHI");
}

#[test]
fn test_delete_across_multiple_pieces() {
    // Start with "ABCDEFGHIJ"
    let mut piece_table = PieceTable::new("ABCDEFGHIJ");
    // Insert "123" after "B" (at position 2): "AB123CDEFGHIJ"
    piece_table.add_text("123", 2).unwrap();
    // Insert "XYZ" after "F" (position 8: 2 for "AB", 3 for "123", 3 for "CDE", so after "F")
    piece_table.add_text("XYZ", 8).unwrap();
    // Now the logical text is: "AB123CDEFXYZGHIJ"
    // Pieces: [AB][123][CDEF][XYZ][GHIJ]
    assert_eq!(piece_table.get_text(), "AB123CDEXYZFGHIJ");

    // Delete from position 3 (the '2' in "123") to position 10 (the 'Y' in "XYZ")
    // This should delete: "23CDEFXY"
    let result = piece_table.delete_text(3, 10 + 1); // end index + 1 for exclusive
    assert!(result.is_ok());

    // The expected logical text is: "AB1ZGHIJ"
    let text = piece_table.get_text();
    assert_eq!(text, "AB1FGHIJ");
}

#[test]
fn test_add_text_across_multiple_pieces() {
    // Start with "ABCDEFGHIJ"
    let mut piece_table = PieceTable::new("ABCDEFGHIJ");
    // Insert "123" after "B" (at position 2): "AB123CDEFGHIJ"
    piece_table.add_text("123", 2).unwrap();
    // Insert "XYZ" after "F" (position 8: 2 for "AB", 3 for "123", 3 for "CDE", so after "F")
    piece_table.add_text("XYZ", 8).unwrap();

    // The expected logical text is: "AB123CDEFXYZGHIJ"
    let text = piece_table.get_text();
    assert_eq!(text, "AB123CDEXYZFGHIJ");

    // Check the pieces for correctness
    let p = &piece_table.pieces;
    assert_eq!(p.len(), 5);

    // [AB][123][CDEF][XYZ][GHIJ]
    assert_eq!(p[0].buffer_type, BufferType::Original);
    assert_eq!(p[0].start, 0);
    assert_eq!(p[0].length, 2);

    assert_eq!(p[1].buffer_type, BufferType::Added);
    assert_eq!(p[1].start, 0);
    assert_eq!(p[1].length, 3);

    assert_eq!(p[2].buffer_type, BufferType::Original);
    assert_eq!(p[2].start, 2);
    assert_eq!(p[2].length, 3);

    assert_eq!(p[3].buffer_type, BufferType::Added);
    assert_eq!(p[3].start, 3);
    assert_eq!(p[3].length, 3);

    assert_eq!(p[4].buffer_type, BufferType::Original);
    assert_eq!(p[4].start, 5);
    assert_eq!(p[4].length, 5);
}
