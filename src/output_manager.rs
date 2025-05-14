use std::io::{self, stdout};

use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};

use crate::content::Content;

pub struct OutputManager;

impl OutputManager {
    pub fn clear_screen() -> io::Result<()> {
        execute!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )
    }

    pub fn refresh_screen(content: &Content) -> io::Result<()> {
        OutputManager::clear_screen()?;
        println!("{}", content.content);
        execute!(
            stdout(),
            cursor::MoveTo(content.cursor_position.x, content.cursor_position.y)
        )
    }
}
