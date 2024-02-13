use ratatui::text::Line;

use crate::lgames::game_manager::Directions;

pub enum Status {
    Original,
    Placed,
    Empty,
}

pub struct Board {
    curr_column: u8,
    curr_line: u8,
    score: u8,
}

impl Board {
    pub fn new() -> Self {
        todo!();
    }

    pub fn reset_board(&mut self) {
        todo!();
    }

    pub fn lost(&self) -> bool {
        todo!();
    }

    pub fn won(&self) -> bool {
        todo!();
    }

    pub fn consult_score(&self) -> u8 {
        todo!();
    }

    pub fn move_cursor(&self, direction: &Directions) {
        todo!();
    }

    pub fn consult_status(&self) -> Status {
        todo!()
    }

    pub fn place(&mut self, value: u8) -> bool {
        todo!();
    }

    pub fn undo(&mut self) -> bool {
        todo!();
    }

    pub fn remove(&mut self) -> bool {
        todo!();
    }

    pub fn display_board(&self, message: String) -> Vec<Line> {
        todo!();
    }
}
