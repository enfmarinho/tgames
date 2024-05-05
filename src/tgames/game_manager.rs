use crossterm::event;
use std::io::Result;

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

// Useful things
pub fn read_key() -> Result<()> {
    event::read()?;
    Ok(())
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

pub enum Difficult {
    Easy,
    Medium,
    Hard,
}
