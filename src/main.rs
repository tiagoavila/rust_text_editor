use std::{io, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};

mod cleanup;
mod content;
mod output_manager;
mod text_trait;
mod piece_table;
mod editor;
mod temporary_buffer_add;
mod temporary_buffer_deletion;
mod enum_add_result;
mod position;

mod prelude {
    pub use crate::cleanup::*;
    pub use crate::output_manager::*;
    pub use crate::text_trait::*;
    pub use crate::piece_table::*;
    pub use crate::editor::*;
    pub use crate::temporary_buffer_add::*;
    pub use crate::temporary_buffer_deletion::*;
    pub use crate::enum_add_result::*;
    pub use crate::position::*;
}

use prelude::*;

fn main() -> io::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    OutputManager::clear_screen()?;
    let initial_text = "Hello World";
    // let initial_text = "Rita Marta";

    // let mut content = Editor::new(String::from("0123456789"), 5);
    let mut content = Editor::new(String::from(initial_text), 5);
    OutputManager::refresh_screen(&content)?;

    loop {
        if poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = read().expect("Failed to read line") {
                let mut stop_loop = false;
                match event {
                    KeyEvent {
                        code: key @ (KeyCode::Char('q') | KeyCode::Esc),
                        modifiers,
                        ..
                    } if key == KeyCode::Esc || (key == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL) => stop_loop = true,
                    KeyEvent {
                        code: key @ (KeyCode::Backspace | KeyCode::Delete),
                        modifiers,
                        ..
                    } => {
                        if key == KeyCode::Delete && modifiers == KeyModifiers::CONTROL {
                            content.delete_word(KeyCode::Delete);
                        } else {
                            content.delete_char(key);
                        }
                    }
                    KeyEvent {
                        code: key @ (KeyCode::Char('h') | KeyCode::Char('w')),
                        modifiers,
                        ..
                    } => {
                        if modifiers == KeyModifiers::CONTROL {
                            content.delete_word(KeyCode::Backspace);
                        } else {
                            content.add_char(key.as_char().unwrap_or(' '));
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        content.add_new_line();
                    }
                    KeyEvent {
                        code: direction @ (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down),
                        ..
                    } => {
                        match direction {
                            KeyCode::Left => content.move_cursor_left(),
                            KeyCode::Right => content.move_cursor_right(),
                            // KeyCode::Up => content.move_cursor_up(),
                            // KeyCode::Down => content.move_cursor_down(),
                            _ => unreachable!(),
                        }
                    }
                    _ => {
                        if let KeyCode::Char(c) = event.code {
                            content.add_char(c);
                        }
                    }
                }
                
                if !stop_loop {
                    OutputManager::refresh_screen(&content)?;
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
