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

mod prelude {
    pub use crate::cleanup::*;
    pub use crate::output_manager::*;
    pub use crate::text_trait::*;
    pub use crate::piece_table::*;
    pub use crate::editor::*;
    pub use crate::temporary_buffer_add::*;
    pub use crate::temporary_buffer_deletion::*;
    pub use crate::enum_add_result::*;
}

use prelude::*;

fn main() -> io::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    OutputManager::clear_screen()?;

    let mut content = Editor::new(String::from("Hello World"), 5);
    OutputManager::refresh_screen(&content)?;

    loop {
        if poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = read().expect("Failed to read line") {
                match event {
                    KeyEvent {
                        code: code @ (KeyCode::Char('q') | KeyCode::Esc),
                        modifiers,
                        ..
                    } if code == KeyCode::Esc || (code == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL) => break,
                    KeyEvent {
                        code: KeyCode::Backspace,
                        ..
                    } => {
                        content.delete_char();
                        OutputManager::refresh_screen(&content)?;
                    }
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        content.add_new_line();
                        OutputManager::refresh_screen(&content)?;
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
                        OutputManager::refresh_screen(&content)?;
                    }
                    _ => {
                        if let KeyCode::Char(c) = event.code {
                            content.add_char(c);
                        }
                        OutputManager::refresh_screen(&content)?;
                    }
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
