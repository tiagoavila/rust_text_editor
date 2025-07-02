use std::io::{self, stdout, Write};

use crossterm::{
    cursor::{self, MoveToColumn, MoveToNextLine}, execute, style::Print, terminal::{self, ClearType}
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
        let mut stdout = stdout();
        for line in content.get_text_lines() {
            execute!(
                stdout,
                Print(format!("{}", line)),
                MoveToNextLine(0), // Move to the next line
                MoveToColumn(0), // Ensure cursor is at column 0
            )
            .unwrap();
        }
        stdout.flush().unwrap(); 
        execute!(stdout, cursor::MoveTo(content.cursor.x, content.cursor.y))
    }
}
