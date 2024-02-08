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
        for index in 0..(NUMBER_OF_LINES * NUMBER_OF_COLUMNS) {
            self.m_board[index] = 0;
        }
        self.m_number_of_moves = 0;
        self.m_score = 0;
        self.m_lost = false;
    }

    pub fn start_game(&mut self) {
        self.generate_piece();
        self.generate_piece();
    }

    pub fn defeated(&self) -> bool {
        self.m_lost
    }

    pub fn move_pieces(&mut self, direction: &Directions) {
        self.m_number_of_moves += 1;
        match direction {
            Directions::Up => self.move_up(),
            Directions::Down => self.move_down(),
            Directions::Right => self.move_right(),
            Directions::Left => self.move_left(),
        }
    }

    pub fn display_board(&self, message: String, color: Color) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line::from(Span::styled(message, color)));
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
                let number_of_spaces: i32;
                if value == 0 {
                    number_of_spaces = 6;
                } else {
                    number_of_spaces = 6 - f32::log10(value as f32).ceil() as i32;
                }
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
        lines
    }

    pub fn consult_score(&self) -> u32 {
        self.m_score
    }

    pub fn consult_number_of_moves(&self) -> u32 {
        self.m_number_of_moves
    }

    fn move_up(&mut self) {
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
