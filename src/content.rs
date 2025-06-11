pub struct Content {
    pub content: String,
    pub cursor_position: usize,
}

impl Content {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.content.push(c);
        self.cursor_position += 1;
    }

    pub fn remove_char(&mut self) {
        if self.cursor_position > 0 {
            self.content.pop();
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        // if self.cursor_position < self.content.len() as u16 {
            // self.cursor_position += 1;
        // }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_down(&mut self) {
        // if self.cursor_position < self.content.len() as u16 {
        //     self.cursor_position += 1;
        // }
    }

    pub fn add_new_line(&mut self) {
        self.content.push('\n');
        self.cursor_position = 0;
    }
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}
