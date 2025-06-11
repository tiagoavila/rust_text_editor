use std::io::{self, stdout};

use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};

use crate::editor::Editor;

pub struct OutputManager;

impl OutputManager {
    pub fn clear_screen() -> io::Result<()> {
        execute!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )
    }

    pub fn refresh_screen(content: &Editor) -> io::Result<()> {
        OutputManager::clear_screen()?;
        println!("{}", content.text);
        execute!(
            stdout(),
            cursor::MoveTo(content.cursor_position as u16, 0)
        )
    }
}
