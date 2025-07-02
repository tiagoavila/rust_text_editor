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
}