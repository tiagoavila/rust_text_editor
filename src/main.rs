use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self},
};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() -> std::io::Result<()> {
    let _guard = CleanUp;
    terminal::enable_raw_mode()?;

    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => break,
                    _ => {
                        //todo
                    }
                }
                println!("{:?}\r", event);
            };
        } else {
            println!("No input yet\r");
        }
    }

    Ok(())

    // let mut buf = [0; 1];
    // let mut text = String::new();
    // while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
    //     // println!("You entered: {}", buf[0] as char);
    //     let character = buf[0] as char;
    //     if character.is_control() {
    //         println!("{}\r", character as u8)
    //     } else {
    //         println!("{}\r", character)
    //     }

    //     text.push(buf[0] as char);
    // }
    // println!("You entered: {}", text);
}
