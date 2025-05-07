use std::time::Duration;

use cleanup::CleanUp;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self},
};
use editor::Editor;

mod cleanup;
mod editor;
mod reader;
mod output;

fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;

    let editor = Editor::new();
    while editor.run()? {}

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
