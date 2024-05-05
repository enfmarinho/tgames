use ratatui::{style::Color, text::Line};

use crate::tgames::game_manager::{Difficult, Directions};

const EASY_BOARD_INFO: BoardInfo = BoardInfo {
    height: 9,
    width: 9,
    number_of_bombs: 10,
};
const MEDIUM_BOARD_INFO: BoardInfo = BoardInfo {
    height: 16,
    width: 16,
    number_of_bombs: 40,
};
const HARD_BOARD_INFO: BoardInfo = BoardInfo {
    height: 16,
    width: 30,
    number_of_bombs: 99,
};

#[derive(Clone)]
struct BoardInfo {
    pub height: usize,
    pub width: usize,
    pub number_of_bombs: usize,
}

struct SquarePosition {
    line: i32,
    column: i32,
}

#[derive(PartialEq)]
enum Square {
    Close(u32),
    Marked(bool),
    Opened(u32),
    Bomb,
}

pub struct Board {
    board: Vec<Vec<Square>>,
    board_info: BoardInfo,
    curr_line: usize,
    curr_column: usize,
    difficult: Difficult,
    marked_squares: usize,
    revealed_bomb: SquarePosition,
    score: usize,
}

impl Board {
    pub fn new() -> Self {
        let mut instance = Self {
            board: Vec::new(),
            board_info: BoardInfo {
                height: 0,
                width: 0,
                number_of_bombs: 0,
            },
            curr_line: 0,
            curr_column: 0,
            difficult: Difficult::Easy,
            marked_squares: 0,
            revealed_bomb: SquarePosition {
                line: -1,
                column: -1,
            },
            score: 0,
        };
        instance.reset(&Difficult::Easy);
        instance
    }

    pub fn won(&self) -> bool {
        todo!()
    }

    pub fn lost(&self) -> bool {
        todo!()
    }

    pub fn reset(&mut self, difficult: &Difficult) {
        match difficult {
            Difficult::Easy => self.board_info = EASY_BOARD_INFO,
            Difficult::Medium => self.board_info = MEDIUM_BOARD_INFO,
            Difficult::Hard => self.board_info = HARD_BOARD_INFO,
        }
        // resize board
        // generate bombs
        // reset curr_line, curr_column, marked_squares and revealed_bomb, score
        todo!()
    }

    pub fn revel(&mut self) {
        if *self.consult_board() == Square::Bomb {
            self.revealed_bomb.line = self.curr_line as i32;
            self.revealed_bomb.column = self.curr_column as i32;
        }
        todo!("Implement algorithm to revel squares")
    }

    pub fn mark(&mut self) {
        match *self.consult_board() {
            Square::Close(_) => {
                self.marked_squares += 1;
                *self.get_position() = Square::Marked(false);
            }
            Square::Bomb => {
                self.marked_squares += 1;
                *self.get_position() = Square::Marked(true);
            }
            Square::Opened(_) | Square::Marked(_) => {}
        }
    }

    pub fn move_cursor(&mut self, direction: &Directions) {
        match direction {
            Directions::Up => {
                if self.curr_line == 0 {
                    self.curr_line = self.board_info.height - 1;
                } else {
                    self.curr_line -= 1;
                }
            }
            Directions::Left => {
                if self.curr_column == 0 {
                    self.curr_column = self.board_info.width - 1;
                } else {
                    self.curr_column -= 1;
                }
            }
            Directions::Down => {
                self.curr_line += 1;
                if self.curr_line == self.board_info.height {
                    self.curr_line = 0;
                }
            }
            Directions::Right => {
                self.curr_line += 1;
                if self.curr_column == self.board_info.width {
                    self.curr_column = 0;
                }
            }
        }
    }

    pub fn display_board(&self, message: &str, color: Color) -> Vec<Line> {
        todo!()
    }

    pub fn score(&self) -> usize {
        self.score
    }

    fn consult_board(&self) -> &Square {
        &self.board[self.curr_line][self.curr_column]
    }

    fn get_position(&mut self) -> &mut Square {
        &mut self.board[self.curr_line][self.curr_column]
    }
}
