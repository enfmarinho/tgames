use rand::{thread_rng, Rng};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

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

#[derive(PartialEq, Eq)]
struct SquarePosition {
    line: i32,
    column: i32,
}
const NOT_REVEALED: SquarePosition = SquarePosition {
    line: -1,
    column: -1,
};

#[derive(PartialEq, Clone)]
enum Square {
    Close(u32),
    Marked(bool),
    Opened(u32),
    Bomb,
}

pub struct Board {
    board: Vec<Square>,
    board_info: BoardInfo,
    curr_line: usize,
    curr_column: usize,
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
            marked_squares: 0,
            revealed_bomb: SquarePosition {
                line: -1,
                column: -1,
            },
            score: 0,
        };
        instance.reset(&Difficult::Medium);
        instance
    }

    pub fn won(&self) -> bool {
        for line in 0..self.board_info.height {
            for column in 0..self.board_info.width {
                if let Square::Close(_) = self.consult_position(line, column) {
                    return false;
                }
            }
        }
        true
    }

    pub fn lost(&self) -> bool {
        self.revealed_bomb != NOT_REVEALED
    }

    pub fn reset(&mut self, difficult: &Difficult) {
        match difficult {
            Difficult::Easy => self.board_info = EASY_BOARD_INFO,
            Difficult::Medium => self.board_info = MEDIUM_BOARD_INFO,
            Difficult::Hard => self.board_info = HARD_BOARD_INFO,
        }
        self.board = vec![Square::Close(0); self.board_info.width * self.board_info.height];
        for _ in 0..self.board_info.number_of_bombs {
            let mut index =
                thread_rng().gen_range(0..self.board_info.height * self.board_info.width);
            while self.board[index] == Square::Bomb {
                index += 1;
                index %= self.board.len();
            }
            self.board[index] = Square::Bomb;
        }
        for line in 0..self.board_info.height {
            for column in 0..self.board_info.width {
                self.update_square_counter(line, column);
            }
        }
        self.curr_line = 0;
        self.curr_column = 0;
        self.marked_squares = 0;
        self.score = 0;
        self.revealed_bomb = NOT_REVEALED;
    }

    fn update_square_counter(&mut self, line: usize, column: usize) {
        match self.consult_position(line, column) {
            Square::Opened(_) | Square::Bomb => return,
            Square::Close(_) | Square::Marked(_) => (),
        }
        let mut counter = 0;
        for line_offset in -1..2 {
            for column_offset in -1..2 {
                let consult_line = line as i32 + line_offset;
                let consult_column = column as i32 + column_offset;
                if self.in_bounds(consult_line, consult_column)
                    && *self.consult_position(consult_line as usize, consult_column as usize)
                        == Square::Bomb
                {
                    counter += 1;
                }
            }
        }
        self.board[line * self.board_info.width + column] = Square::Close(counter);
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
            Square::Marked(correct) => {
                self.marked_squares -= 1;
                if correct {
                    *self.get_position() = Square::Bomb;
                } else {
                    self.update_square_counter(self.curr_line, self.curr_column);
                }
            }
            Square::Opened(_) => (),
        }
    }

    pub fn revel(&mut self) {
        let revealed = match *self.get_position() {
            Square::Bomb => {
                self.revealed_bomb.line = self.curr_line as i32;
                self.revealed_bomb.column = self.curr_column as i32;
                true
            }
            Square::Close(amount) => {
                *self.get_position() = Square::Opened(amount);
                self.score += 1;
                true
            }
            Square::Opened(_) | Square::Marked(_) => false,
        };
        if revealed {
            // todo!("Implement algorithm to revel squares")
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
                self.curr_column += 1;
                if self.curr_column == self.board_info.width {
                    self.curr_column = 0;
                }
            }
        }
    }

    pub fn display_board(&self, message: String, color: Color) -> Vec<Line> {
        let mut lines = Vec::new();
        lines.push(Line::from(Span::styled(message, color)));
        self.push_horizontal_board(&mut lines, true);
        for line in 0..self.board_info.height {
            let mut spans = Vec::new();
            spans.push(Span::styled("│ ", Style::default().fg(Color::DarkGray)));
            for column in 0..self.board_info.width {
                if line as i32 == self.revealed_bomb.line
                    && column as i32 == self.revealed_bomb.column
                {
                    spans.push(Span::styled(
                        "󰚑 ",
                        Style::default().fg(Color::Black).bg(Color::LightRed),
                    ));
                    continue;
                }
                let background_color = if line == self.curr_line && column == self.curr_column {
                    Color::Gray
                } else {
                    Color::Reset
                };
                match *self.consult_position(line, column) {
                    Square::Opened(amount) => {
                        let color = match amount {
                            1 => Color::LightBlue,
                            2 => Color::LightGreen,
                            3 => Color::LightRed,
                            4 => Color::Blue,
                            5 => Color::Red,
                            _ => Color::Black,
                        };
                        let char = if amount == 0 {
                            "  ".to_string()
                        } else {
                            ((amount as u8 + b'0') as char).to_string() + " "
                        };
                        spans.push(Span::styled(
                            char,
                            Style::default().fg(color).bg(background_color),
                        ));
                    }
                    Square::Marked(correct) => {
                        if self.revealed_bomb == NOT_REVEALED || correct {
                            spans.push(
                                Span::styled("󰈿 ", Style::default().fg(Color::Red))
                                    .bg(background_color),
                            );
                        } else {
                            spans.push(
                                Span::styled("󰛅 ", Style::default().fg(Color::DarkGray))
                                    .bg(background_color),
                            );
                        }
                    }
                    Square::Close(_) => {
                        spans.push(
                            Span::styled(" ", Style::default().fg(Color::DarkGray))
                                .bg(background_color),
                        );
                    }
                    Square::Bomb => {
                        if self.revealed_bomb != NOT_REVEALED {
                            spans.push(Span::styled("󰚑 ", Style::default().fg(Color::LightRed)));
                        } else {
                            spans.push(
                                Span::styled(" ", Style::default().fg(Color::DarkGray))
                                    .bg(background_color),
                            );
                        }
                    }
                }
            }
            spans.push(Span::styled(" │", Style::default().fg(Color::DarkGray)));
            lines.push(Line::from(spans));
            let mut empty_line = String::from('│');
            for _ in 0..self.board_info.width * 2 {
                empty_line.push(' ');
            }
            empty_line.push('│');
            // lines.push(Line::from(Span::styled(
            //     empty_line,
            //     Style::default().fg(Color::DarkGray),
            // )));
        }

        self.push_horizontal_board(&mut lines, false);
        lines
    }

    fn push_horizontal_board(&self, lines: &mut Vec<Line>, up: bool) {
        let mut spans: String = String::new();
        if up {
            spans += "╭";
        } else {
            spans += "╰";
        }
        for _ in 0..self.board_info.width * 2 + 2 {
            spans += "─";
        }
        if up {
            spans += "╮";
        } else {
            spans += "╯";
        }
        lines.push(Line::from(Span::styled(
            spans,
            Style::default().fg(Color::DarkGray),
        )));
    }

    pub fn score(&self) -> usize {
        self.score
    }

    fn in_bounds(&self, line: i32, column: i32) -> bool {
        line >= 0
            && line < self.board_info.height as i32
            && column >= 0
            && column < self.board_info.width as i32
    }

    fn consult_position(&self, line: usize, column: usize) -> &Square {
        &self.board[line * self.board_info.width + column]
    }

    fn consult_board(&self) -> &Square {
        &self.board[self.curr_line * self.board_info.width + self.curr_column]
    }

    fn get_position(&mut self) -> &mut Square {
        &mut self.board[self.curr_line * self.board_info.width + self.curr_column]
    }
}
