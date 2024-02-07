use super::super::game_manager::Directions;
use ratatui::text::Line;

const NUMBER_OF_COLUMNS: usize = 4;
const NUMBER_OF_LINES: usize = 4;

pub struct Board {
    m_board: Vec<u32>,
    m_moves: u32,
    m_score: u32,
}

impl Board {
    pub fn new() -> Self {
        todo!();
    }

    pub fn reset_board(&mut self) {
        todo!();
    }

    pub fn defeated(&self) -> bool {
        todo!();
    }

    pub fn move_pieces(&mut self, direction: &Directions) {
        todo!();
    }

    pub fn display_board(&self, message: &str) -> Vec<Line> {
        todo!();
    }

    pub fn consult_score(&self) -> u32 {
        todo!();
    }

    pub fn consult_number_of_moves(&self) -> u32 {
        todo!();
    }

    fn consult_board(&self, line: usize, column: usize) -> u32 {
        self.m_board[line * NUMBER_OF_COLUMNS + column]
    }
}
