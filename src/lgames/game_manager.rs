use crossterm::event;
// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::Result;

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

pub trait GameManager {
    fn run(&mut self) -> Result<()> {
        while !self.ended() {
            self.process_events()?;
            self.update()?;
            self.render()?;
            self.limit_fps();
        }
        Ok(())
    }
    fn process_events(&mut self) -> Result<()>;
    fn update(&mut self) -> Result<()>;
    fn render(&mut self) -> Result<()>;
    fn ended(&self) -> bool;
    // In case is needed to limit game fps.
    fn limit_fps(&self) {}
}

// Useful functions
// pub fn read_confirmation() -> bool {
//     let event = event::read();
//     let key: Event;
//     match event {
//         Ok(value) => key = value,
//         Err(_) => return false,
//     }
//     loop {
//         match key {
//             Event::Key(KeyEvent {
//                 code: KeyCode::Char('y'),
//                 kind: KeyEventKind::Press,
//                 ..
//             }) => return true,
//             Event::Key(KeyEvent {
//                 code: KeyCode::Char('n'),
//                 kind: KeyEventKind::Press,
//                 ..
//             }) => return false,
//             _ => (),
//         }
//     }
// }

pub fn read_key() -> Result<()> {
    let event = event::read()?;
    match event {
        _ => Ok(()),
    }
}
