use std::io::{self, stdout, Write};

use crossterm::{
    cursor::{self, MoveTo, MoveToColumn, MoveToNextLine},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, size, Clear, ClearType},
};

use crate::core::editor::Editor;

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
                MoveToColumn(0),   // Ensure cursor is at column 0
            )
            .unwrap();
        }

        let text = content.get_text();
        let (width, height) = size().unwrap();

        // Draw the bottom border with ~~~~~~~~~~~~~~~~
        execute!(
            stdout,
            MoveTo(0, height - 5),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print("~".repeat(width as usize)), // ~~~~~~~~~~~~~~~~
            ResetColor,
        )
        .unwrap();

        // Display the text, cursor position, length, and console size
        execute!(
            stdout,
            MoveTo(0, height - 4),
            SetForegroundColor(Color::Cyan),
            // Print(format!("Text: {:?}", text)),
            MoveTo(0, height - 3),
            SetForegroundColor(Color::Yellow),
            Print(format!(
                "Cursor: (row: {}, col: {})",
                content.cursor.y, content.cursor.x
            )),
            MoveTo(0, height - 2),
            SetForegroundColor(Color::Green),
            Print(format!("Length: {} characters", text.len())),
            MoveTo(0, height - 1),
            SetForegroundColor(Color::Blue),
            Print(format!(
                "Console size: width - {} height - {}",
                width, height
            )),
            ResetColor,
            MoveTo(content.cursor.x, content.cursor.y), // Move back to your app's cursor position
        )
        .unwrap();
        stdout.flush().unwrap();
        execute!(stdout, cursor::MoveTo(content.cursor.x, content.cursor.y))
    }
}
