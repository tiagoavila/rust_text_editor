# Text Editor in Rust

![Text Editor Logo](src/images/logo.png)

A high-performance text editor written in Rust that implements efficient text manipulation using a piece table data structure. This editor provides a terminal-based interface with advanced text editing capabilities.

> **Inspired by**: This project was motivated by [Austin Henley's "Challenging projects every programmer should try"](https://austinhenley.com/blog/challengingprojects.html), which highlights text editor implementation as one of the most educational programming projects. Building a text editor from scratch teaches fundamental concepts about data structures, cursor behavior, and efficient text manipulation that every programmer should understand.

## Features

### Core Architecture
- **Piece Table Implementation**: Uses a piece table data structure for efficient text storage and manipulation, allowing for fast insertions and deletions without moving large amounts of data
- **Dual Buffer System**: Implements both add and delete temporary buffers for optimized text operations
- **Terminal-based Interface**: Built with `crossterm` for cross-platform terminal support

### Text Editing Features
- **Character Insertion**: Add characters at cursor position with automatic buffer management
- **Character Deletion**: Support for both Backspace and Delete keys
- **Word Deletion**: Ctrl+Backspace and Ctrl+Delete for word-level deletion
- **New Line Support**: Enter key to add new lines with proper cursor positioning
- **Cursor Navigation**: Arrow keys for moving cursor left and right (up/down navigation prepared for future implementation)

### Advanced Buffer Management
- **Temporary Add Buffer**: Efficiently batches character insertions before persisting to the piece table
- **Temporary Delete Buffer**: Batches deletions for optimal performance
- **Smart Persistence**: Buffers are automatically persisted when they reach capacity or when operations require it
- **Position Tracking**: Maintains accurate cursor position across all operations

### Performance Optimizations
- **Lazy Persistence**: Text changes are batched in temporary buffers to reduce piece table operations
- **Efficient Piece Management**: The piece table minimizes data movement during insertions and deletions
- **Memory Efficient**: Only stores original text and additions, avoiding redundant data storage

### User Interface
- **Real-time Screen Updates**: Immediate visual feedback for all text operations
- **Clean Terminal Interface**: Proper screen clearing and cursor positioning
- **Cross-platform Support**: Works on Windows, macOS, and Linux terminals

## Technical Details

### Piece Table Implementation
The editor uses a piece table data structure that consists of:
- **Original Buffer**: Contains the initial text
- **Add Buffer**: Stores all text additions
- **Pieces**: References that map to specific ranges in either buffer

### Buffer System
- **Add Buffer**: Temporarily stores new characters before persisting to the piece table
- **Delete Buffer**: Tracks deletions before applying them to the piece table
- **Smart Flushing**: Buffers are automatically persisted when they reach optimal size

## Usage

### Building
```bash
cargo build --release
```

### Running
```bash
cargo run
```

### Controls
- **Character Input**: Type normally to add characters
- **Navigation**: Use arrow keys to move cursor
- **Deletion**: 
  - `Backspace`: Delete character before cursor
  - `Delete`: Delete character after cursor
  - `Ctrl+Backspace`: Delete word before cursor
  - `Ctrl+Delete`: Delete word after cursor
- **New Line**: `Enter` to add new line
- **Exit**: `Ctrl+Q` or `Esc` to quit

## Project Structure

```
src/
├── main.rs                 # Main application entry point
├── editor.rs              # Core editor logic and state management
├── piece_table.rs         # Piece table data structure implementation
├── temporary_buffer_add.rs # Add buffer management
├── temporary_buffer_deletion.rs # Delete buffer management
├── output_manager.rs      # Terminal output and screen management
├── position.rs            # Cursor position tracking
├── cleanup.rs             # Terminal cleanup utilities
├── text_trait.rs          # Text manipulation trait definitions
├── enum_add_result.rs     # Result types for buffer operations
└── images/
    └── logo.png           # Project logo
```

## Dependencies

- **crossterm**: Cross-platform terminal manipulation library
- **std**: Rust standard library for core functionality

## Development

This project demonstrates advanced text editor concepts including:
- Efficient data structures for text manipulation
- Terminal UI programming
- Buffer management and optimization
- Rust's memory safety and performance characteristics

## License

This project is open source and available under the MIT License. 