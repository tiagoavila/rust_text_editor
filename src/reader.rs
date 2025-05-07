use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent};

pub(crate) struct Reader;

/* add the following*/
impl Reader {
    pub fn read_key(&self) -> std::io::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}