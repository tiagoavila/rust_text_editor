pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        self.x += 1; // Assuming no limit for simplicity
    }
    
    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        self.y += 1; // Assuming no limit for simplicity
    }
    
    pub fn move_to_new_line(&mut self) {
        self.x = 0;
        self.y += 1; // Move to the next line
    }
}