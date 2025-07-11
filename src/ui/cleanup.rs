
use crossterm::
    terminal::{self}
;

use crate::ui::output_manager::OutputManager;

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        OutputManager::clear_screen().expect("Could not clear screen");
    }
}


