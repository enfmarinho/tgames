use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use std::io::Result;

pub trait GameManager {
    fn run(&mut self) -> Result<bool> {
        while !self.ended() {
            self.process_events()?;
            self.update()?;
            self.render()?;
            self.limit_fps();
        }
        Ok(self.kill_execution())
    }
    fn process_events(&mut self) -> Result<()>;
    fn update(&mut self) -> Result<()>;
    fn render(&mut self) -> Result<()>;
    fn ended(&self) -> bool;
    fn kill_execution(&self) -> bool;
    // In case is needed to limit game fps.
    fn limit_fps(&self) {}
}

// Useful things
pub fn read_key() -> Result<()> {
    event::read()?;
    Ok(())
}

pub fn read_confirmation(key: &Event) -> bool {
    !matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Char('N'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn confirmation_guide() -> String {
    String::from("N or n - go back to playing\nAny key - confirm")
}

#[derive(Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub enum Difficult {
    Easy,
    Medium,
    Hard,
}
pub fn force_quit(key: &Event) -> bool {
    *key == Event::Key(KeyEvent {
        code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::all(),
    })
}
