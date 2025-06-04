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
                    start: end_piece.start + end_offset + 1, // Skip the deleted part
                    length: end_piece.length - end_offset - 1, // Remaining length
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
