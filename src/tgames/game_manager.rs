use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::Result;
use std::io::Stdout;

pub trait GameManager {
    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<bool> {
        while !self.ended() {
            self.process_events()?;
            self.update()?;
            self.render(terminal)?;
            self.limit_fps();
        }
        self.reset();
        Ok(self.kill_execution())
    }
    fn process_events(&mut self) -> Result<()>;
    fn update(&mut self) -> Result<()>;
    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()>;
    fn reset(&mut self);
    fn ended(&self) -> bool;
    fn kill_execution(&self) -> bool;
    // In case is needed to limit game fps.
    fn limit_fps(&self) {}
}

// Useful things
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
