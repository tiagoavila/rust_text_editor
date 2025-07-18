use clap::Parser;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};
use std::{fs, io, path::PathBuf, time::Duration};

mod core {
    pub mod editor;
    pub mod piece_table;
    pub mod position;
    pub mod text_trait;
}
mod buffer {
    pub mod temporary_buffer_add;
    pub mod temporary_buffer_deletion;
}
mod ui {
    pub mod output_manager;
    pub mod cleanup;
}
mod enums {
    pub mod enum_add_result;
    pub mod text_action;
}

mod prelude {
    pub use crate::buffer::temporary_buffer_add::*;
    pub use crate::buffer::temporary_buffer_deletion::*;
    pub use crate::core::editor::*;
    pub use crate::core::piece_table::*;
    pub use crate::core::position::*;
    pub use crate::core::text_trait::*;
    pub use crate::enums::enum_add_result::*;
    pub use crate::ui::cleanup::*;
    pub use crate::ui::output_manager::*;
    pub use crate::enums::text_action::*;
}

use prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Use single line text
    #[arg(long)]
    single: bool,

    /// Use multiple lines text
    #[arg(long)]
    multi: bool,

    /// Load text from file
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    OutputManager::clear_screen()?;

    let single_line_text = "Hello World";
    let multiple_lines_text = "Hello World\nThis is a text editor\nIt supports multiple lines\nAnd basic editing features";

    // Default to single if no parameter is passed
    let initial_text = if args.single || (!args.single && !args.multi && args.file.is_none()) {
        single_line_text.to_string()
    } else if args.multi {
        multiple_lines_text.to_string()
    } else if let Some(path) = args.file {
        fs::read_to_string(path).unwrap_or_else(|_| String::from("file not found"))
    } else {
        single_line_text.to_string()
    };

    let mut editor = Editor::new(initial_text, 5);
    OutputManager::refresh_screen(&editor)?;

    loop {
        if poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = read().expect("Failed to read line") {
                let mut stop_loop = false;
                match event {
                    KeyEvent {
                        code: key @ (KeyCode::Char('q') | KeyCode::Esc),
                        modifiers,
                        ..
                    } if key == KeyCode::Esc
                        || (key == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL) =>
                    {
                        stop_loop = true
                    }
                    KeyEvent {
                        code: key @ (KeyCode::Backspace | KeyCode::Delete),
                        modifiers,
                        ..
                    } => {
                        if key == KeyCode::Delete && modifiers == KeyModifiers::CONTROL {
                            editor.delete_word(KeyCode::Delete);
                        } else {
                            editor.delete_char(key);
                        }
                    }
                    KeyEvent {
                        code: key @ (KeyCode::Char('h') | KeyCode::Char('w')),
                        modifiers,
                        ..
                    } => {
                        if modifiers == KeyModifiers::CONTROL {
                            editor.delete_word(KeyCode::Backspace);
                        } else {
                            editor.add_char(key.as_char().unwrap_or(' '));
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        editor.add_new_line();
                    }
                    KeyEvent {
                        code:
                            direction @ (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down),
                        ..
                    } => match direction {
                        KeyCode::Left => editor.move_cursor_left(),
                        KeyCode::Right => editor.move_cursor_right(),
                        KeyCode::Up => editor.move_cursor_up(),
                        KeyCode::Down => editor.move_cursor_down(),
                        _ => unreachable!(),
                    },
                    _ => {
                        if event.code == KeyCode::Char('z') && event.modifiers == KeyModifiers::CONTROL {
                            editor.undo_change();
                        } else if let KeyCode::Char(c) = event.code {
                            editor.add_char(c);
                        }
                    }
                }

                if !stop_loop {
                    OutputManager::refresh_screen(&editor)?;
                } else {
                    break;
                }
            };
        } else {
            // Timeout expired, no `Event` is available
            // content.persist_temporary_buffer();
            // OutputManager::refresh_screen(&content)?;
        }
    }

    Ok(())
}
