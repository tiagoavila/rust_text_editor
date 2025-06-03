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
    let result = piece_table.delete_text(3, 6);

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
    let result = piece_table.delete_text(6, 9);

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
    // let result = piece_table.delete_text(6, 9);

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
    let result = piece_table.delete_text(9, 12);

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
    let result = piece_table.delete_text(0, 3);

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
    let result = piece_table.delete_text(3, 6);

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

    // Delete from position 3 (the '2' in "123") to position 10 (the 'Y' in "XYZ")
    // This should delete: "23CDEFXY"
    let result = piece_table.delete_text(3, 10);
    assert!(result.is_ok());

    // The expected logical text is: "AB1ZGHIJ"
    let text = piece_table.get_text();
    assert_eq!(text, "AB1ZGHIJ");
}