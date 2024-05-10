use super::super::game_manager::Directions;
use rand::Rng;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

const NUMBER_OF_COLUMNS: usize = 4;
const NUMBER_OF_LINES: usize = 4;

pub struct Board {
    board: Vec<u32>,
    number_of_moves: u32,
    score: u32,
    lost: bool,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            board: Vec::with_capacity(NUMBER_OF_LINES * NUMBER_OF_COLUMNS),
            number_of_moves: 0,
            score: 0,
            lost: false,
        };
        board.board.resize(NUMBER_OF_LINES * NUMBER_OF_COLUMNS, 0);
        board.start_game();
        board
    }

    pub fn reset_board(&mut self) {
        for index in 0..(NUMBER_OF_LINES * NUMBER_OF_COLUMNS) {
            self.board[index] = 0;
        }
        self.number_of_moves = 0;
        self.score = 0;
        self.lost = false;
    }

    pub fn start_game(&mut self) {
        self.generate_block();
        self.generate_block();
    }

    pub fn defeated(&self) -> bool {
        self.lost
    }

    pub fn move_pieces(&mut self, direction: &Directions) {
        if match direction {
            Directions::Up => self.move_up(),
            Directions::Down => self.move_down(),
            Directions::Right => self.move_right(),
            Directions::Left => self.move_left(),
        } {
            self.number_of_moves += 1;
            self.generate_block();
            self.lost = self.cannot_move();
        }
    }

    pub fn display_board(&self, message: String, color: Color) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let mut spans: String = String::new();
        spans += "╭";
        for counter in 1..NUMBER_OF_COLUMNS * 7 {
            if counter % 7 == 0 {
                spans += "┬";
            } else {
                spans += "─";
            }
        }
        spans += "╮";
        lines.push(Line::from(Span::styled(
            spans,
            Style::default().fg(Color::DarkGray),
        )));
        let mut separator_line: String = String::new();
        separator_line += "│";
        for counter in 1..NUMBER_OF_COLUMNS * 7 {
            if counter % 7 == 0 {
                separator_line += "┼";
            } else {
                separator_line += "─";
            }
        }
        separator_line += "│";
        for line in 0..NUMBER_OF_LINES {
            if line != 0 {
                lines.push(Line::from(Span::styled(
                    separator_line.clone(),
                    Style::default().fg(Color::DarkGray),
                )));
            }
            let mut spans: Vec<Span> = Vec::new();

            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            for column in 0..NUMBER_OF_COLUMNS {
                let value = self.consult_board(line, column);
                let number_of_spaces = if value == 0 {
                    6
                } else {
                    6 - f32::log10(value as f32).ceil() as i32
                };
                let mut word = String::new();
                for _ in 0..number_of_spaces {
                    word.push(' ');
                }
                if value != 0 {
                    word += &value.to_string();
                }
                match value {
                    2 | 128 | 8192 => {
                        spans.push(Span::styled(word, Style::default().fg(Color::Red)));
                    }
                    4 | 256 | 16384 => {
                        spans.push(Span::styled(word, Style::default().fg(Color::Yellow)));
                    }
                    8 | 512 | 32768 => {
                        spans.push(Span::styled(word, Style::default().fg(Color::Magenta)));
                    }
                    16 | 1024 | 65536 => {
                        spans.push(Span::styled(word, Style::default().fg(Color::Blue)));
                    }
                    32 | 2048 | 131072 => {
                        spans.push(Span::styled(word, Style::default().fg(Color::Cyan)));
                    }
                    64 | 4096 => {
                        spans.push(Span::styled(word, Style::default().fg(Color::Green)));
                    }
                    _ => {
                        spans.push(Span::styled(word, Style::default()));
                    }
                }
                spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            }
            lines.push(Line::from(spans));
        }
        let mut spans: String = String::new();
        spans += "╰";
        for counter in 1..NUMBER_OF_COLUMNS * 7 {
            if counter % 7 == 0 {
                spans += "┴";
            } else {
                spans += "─";
            }
        }
        spans += "╯";
        lines.push(Line::from(Span::styled(
            spans,
            Style::default().fg(Color::DarkGray),
        )));
        if !message.is_empty() {
            lines.push(Line::from(Span::styled(message, color)));
            lines.push(Line::from(Span::styled(
                "Press enter to play again.",
                Style::default(),
            )));
        }
        lines
    }

    pub fn consult_score(&self) -> u32 {
        self.score
    }

    pub fn consult_number_of_moves(&self) -> u32 {
        self.number_of_moves
    }

    fn move_up(&mut self) -> bool {
        let mut moved: bool = false;
        for column in 0..NUMBER_OF_COLUMNS {
            for line in 1..NUMBER_OF_LINES {
                let value = self.consult_board(line, column);
                if value == 0 {
                    continue;
                }
                let mut index = line - 1;
                while index > 0 && self.consult_board(index, column) == 0 {
                    index -= 1;
                }
                if value == self.consult_board(index, column) {
                    *self.get_board_pos(index, column) = 2 * value;
                    *self.get_board_pos(line, column) = 0;
                    self.score += 2 * value;
                    moved = true;
                } else if self.consult_board(index, column) == 0 {
                    *self.get_board_pos(index, column) = value;
                    *self.get_board_pos(line, column) = 0;
                    moved = true;
                } else if index + 1 != line {
                    *self.get_board_pos(index + 1, column) = value;
                    *self.get_board_pos(line, column) = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved: bool = false;
        for column in 0..NUMBER_OF_COLUMNS {
            for diff in 1..NUMBER_OF_LINES {
                let line = NUMBER_OF_LINES - diff - 1;
                let value = self.consult_board(line, column);
                if value == 0 {
                    continue;
                }
                let mut index = line + 1;
                while index < NUMBER_OF_LINES - 1 && self.consult_board(index, column) == 0 {
                    index += 1;
                }
                if value == self.consult_board(index, column) {
                    *self.get_board_pos(index, column) = 2 * value;
                    *self.get_board_pos(line, column) = 0;
                    self.score += 2 * value;
                    moved = true;
                } else if self.consult_board(index, column) == 0 {
                    *self.get_board_pos(index, column) = value;
                    *self.get_board_pos(line, column) = 0;
                    moved = true;
                } else if index - 1 != line {
                    *self.get_board_pos(index - 1, column) = value;
                    *self.get_board_pos(line, column) = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn move_right(&mut self) -> bool {
        let mut moved: bool = false;
        for line in 0..NUMBER_OF_LINES {
            for diff in 1..NUMBER_OF_COLUMNS {
                let column = NUMBER_OF_COLUMNS - diff - 1;
                let value = self.consult_board(line, column);
                if value == 0 {
                    continue;
                }
                let mut index = column + 1;
                while index < NUMBER_OF_COLUMNS - 1 && self.consult_board(line, index) == 0 {
                    index += 1;
                }
                if value == self.consult_board(line, index) {
                    *self.get_board_pos(line, index) = 2 * value;
                    *self.get_board_pos(line, column) = 0;
                    self.score += 2 * value;
                    moved = true;
                } else if self.consult_board(line, index) == 0 {
                    *self.get_board_pos(line, index) = value;
                    *self.get_board_pos(line, column) = 0;
                    moved = true;
                } else if index - 1 != column {
                    *self.get_board_pos(line, index - 1) = value;
                    *self.get_board_pos(line, column) = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved: bool = false;
        for line in 0..NUMBER_OF_LINES {
            for column in 1..NUMBER_OF_COLUMNS {
                let value = self.consult_board(line, column);
                if value == 0 {
                    continue;
                }
                let mut index = column - 1;
                while index > 0 && self.consult_board(line, index) == 0 {
                    index -= 1;
                }
                if value == self.consult_board(line, index) {
                    *self.get_board_pos(line, index) = 2 * value;
                    *self.get_board_pos(line, column) = 0;
                    self.score += 2 * value;
                    moved = true;
                } else if self.consult_board(line, index) == 0 {
                    *self.get_board_pos(line, index) = value;
                    *self.get_board_pos(line, column) = 0;
                    moved = true;
                } else if index + 1 != column {
                    *self.get_board_pos(line, index + 1) = value;
                    *self.get_board_pos(line, column) = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn generate_block(&mut self) {
        let mut index = rand::thread_rng().gen_range(0..(NUMBER_OF_LINES * NUMBER_OF_COLUMNS));
        while self.board[index] != 0 {
            index += 1;
            index %= NUMBER_OF_LINES * NUMBER_OF_COLUMNS;
        }
        let random = rand::thread_rng().gen_range(0..10);
        if random == 4 {
            self.board[index] = 4;
        } else {
            self.board[index] = 2;
        }
    }

    fn consult_board(&self, line: usize, column: usize) -> u32 {
        self.board[line * NUMBER_OF_COLUMNS + column]
    }

    fn get_board_pos(&mut self, line: usize, column: usize) -> &mut u32 {
        &mut self.board[line * NUMBER_OF_COLUMNS + column]
    }

    fn cannot_move(&self) -> bool {
        for line in 0..NUMBER_OF_LINES - 1 {
            for column in 0..NUMBER_OF_COLUMNS - 1 {
                if self.consult_board(line, column) == 0
                    || self.consult_board(line, column) == self.consult_board(line, column + 1)
                    || self.consult_board(line, column) == self.consult_board(line + 1, column)
                {
                    return false;
                }
            }
        }
        for line in 0..NUMBER_OF_LINES - 1 {
            if self.consult_board(line, NUMBER_OF_COLUMNS - 1) == 0
                || self.consult_board(line, NUMBER_OF_COLUMNS - 1)
                    == self.consult_board(line + 1, NUMBER_OF_COLUMNS - 1)
            {
                return false;
            }
        }
        for column in 0..NUMBER_OF_COLUMNS - 1 {
            if self.consult_board(NUMBER_OF_LINES - 1, column) == 0
                || self.consult_board(NUMBER_OF_LINES - 1, column)
                    == self.consult_board(NUMBER_OF_LINES - 1, column + 1)
            {
                return false;
            }
        }
        self.consult_board(NUMBER_OF_LINES - 1, NUMBER_OF_COLUMNS - 1) != 0
    }
}
