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
    number_of_bombs: 100,
};

#[derive(Clone)]
struct BoardInfo {
    pub height: usize,
    pub width: usize,
    pub number_of_bombs: i32,
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
    Close(usize),
    Opened(usize),
    Marked(bool),
    Uncertain(bool),
    Bomb,
}

pub struct Board {
    board: Vec<Square>,
    board_info: BoardInfo,
    hide_cursor: bool,
    curr_line: usize,
    curr_column: usize,
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
            hide_cursor: false,
            curr_line: 0,
            curr_column: 0,
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
                if let Square::Close(_) = *self.consult_position(line, column) {
                    return false;
                } else if let Square::Bomb = *self.consult_position(line, column) {
                    return false;
                } else if let Square::Uncertain(_) = *self.consult_position(line, column) {
                    return false;
                }
            }
        }
        true
    }

    pub fn lost(&self) -> bool {
        self.revealed_bomb != NOT_REVEALED
    }

    pub fn bombs(&self) -> i32 {
        self.board_info.number_of_bombs
    }

    pub fn clear(&mut self) {
        self.board.fill(Square::Close(0));
        self.hide_cursor = true;
    }

    pub fn reset(&mut self, difficult: &Difficult) {
        self.score = 0;
        self.hide_cursor = false;
        self.revealed_bomb = NOT_REVEALED;
        match difficult {
            Difficult::Easy => self.board_info = EASY_BOARD_INFO,
            Difficult::Medium => self.board_info = MEDIUM_BOARD_INFO,
            Difficult::Hard => self.board_info = HARD_BOARD_INFO,
        }
        self.board = vec![Square::Close(0); self.board_info.width * self.board_info.height];
        for _ in 0..self.board_info.number_of_bombs {
            let mut index = thread_rng().gen_range(0..self.board.len());
            while self.board[index] == Square::Bomb {
                index += 1;
                index %= self.board.len();
            }
            self.board[index] = Square::Bomb;
        }
        for line in 0..self.board_info.height {
            for column in 0..self.board_info.width {
                self.update_close_square_counter(line, column);
            }
        }
        self.curr_line = thread_rng().gen_range(0..self.board_info.height);
        self.curr_column = thread_rng().gen_range(0..self.board_info.width);
        let mut counter = 0;
        while *self.consult_board() != Square::Close(0) {
            self.curr_column += 1;
            self.curr_column %= self.board_info.width;
            counter += 1;
            if counter == self.board_info.width {
                counter = 0;
                self.curr_line += 1;
                self.curr_line %= self.board_info.height;
            }
        }
        self.reveal_block(self.curr_line, self.curr_column);
    }

    fn update_close_square_counter(&mut self, line: usize, column: usize) {
        if let Square::Close(_) = *self.consult_position(line, column) {
            let mut counter = 0;
            for line_offset in -1..=1 {
                for column_offset in -1..=1 {
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
    }

    pub fn mark(&mut self) {
        match *self.consult_board() {
            Square::Close(_) => {
                self.board_info.number_of_bombs -= 1;
                *self.get_position() = Square::Marked(false);
            }
            Square::Bomb => {
                self.board_info.number_of_bombs -= 1;
                *self.get_position() = Square::Marked(true);
            }
            Square::Marked(correct) => {
                self.board_info.number_of_bombs += 1;
                *self.get_position() = Square::Uncertain(correct);
            }
            Square::Uncertain(correct) => match correct {
                true => *self.get_position() = Square::Bomb,
                false => {
                    *self.get_position() = Square::Close(0);
                    self.update_close_square_counter(self.curr_line, self.curr_column);
                }
            },
            Square::Opened(_) => (),
        }
    }

    pub fn reveal_block(&mut self, line: usize, column: usize) {
        self.score += 1;
        let amount = if let Square::Close(amount) = *self.consult_position(line, column) {
            amount
        } else {
            return;
        };
        self.board[line * self.board_info.width + column] = Square::Opened(amount);
        if amount != 0 {
            return;
        }
        let offsets = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        for (vertical_offset, horizontal_offset) in offsets {
            let line = line as i32 + vertical_offset;
            let column = column as i32 + horizontal_offset;
            if self.in_bounds(line, column) {
                self.reveal_block(line as usize, column as usize);
            }
        }
    }

    pub fn reveal(&mut self) {
        match *self.get_position() {
            Square::Close(_) => self.reveal_block(self.curr_line, self.curr_column),
            Square::Bomb => {
                self.revealed_bomb.line = self.curr_line as i32;
                self.revealed_bomb.column = self.curr_column as i32;
            }
            Square::Opened(_) | Square::Marked(_) | Square::Uncertain(_) => (),
        };
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
                let background_color =
                    if line == self.curr_line && column == self.curr_column && !self.hide_cursor {
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
                            6 => Color::Yellow,
                            _ => Color::Magenta,
                        };
                        let amount = if amount == 0 {
                            "  ".to_string()
                        } else {
                            amount.to_string() + " "
                        };
                        spans.push(Span::styled(
                            amount,
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
                    Square::Uncertain(correct) => {
                        if self.revealed_bomb != NOT_REVEALED && !correct {
                            spans.push(
                                Span::styled("󰛅 ", Style::default().fg(Color::DarkGray))
                                    .bg(background_color),
                            );
                        } else {
                            spans.push(
                                Span::styled("? ", Style::default().fg(Color::Magenta))
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
        }
        self.push_horizontal_board(&mut lines, false);
        lines.push(Line::from(Span::styled(message, color)));
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
