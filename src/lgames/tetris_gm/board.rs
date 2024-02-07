use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::bricks::Brick;
use super::game_manager::Coord;

const NUMBER_OF_LINES: usize = 20;
const NUMBER_OF_COLUMNS: usize = 10;

#[derive(Clone, Copy)]
pub enum BoardPossibilities {
    Orange,
    Blue,
    Cyan,
    Green,
    Yellow,
    Red,
    Pink,
    Empty,
}

pub struct Board {
    m_board: Vec<Vec<BoardPossibilities>>,
    m_lost: bool,
    m_score: u32,
    m_lines: u32,
    m_brick: Brick,
    m_next_brick: Brick,
    m_brick_coord: Coord,
}
impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            m_board: Vec::with_capacity(NUMBER_OF_LINES),
            m_lost: false,
            m_score: 0,
            m_lines: 0,
            m_brick: Brick::new(),
            m_next_brick: Brick::new(),
            m_brick_coord: Coord {
                x: NUMBER_OF_COLUMNS / 2,
                y: 0,
            },
        };
        let mut line: Vec<BoardPossibilities> = Vec::with_capacity(NUMBER_OF_COLUMNS);
        for _counter in 0..NUMBER_OF_COLUMNS {
            line.push(BoardPossibilities::Empty);
        }
        board.m_board.resize(
            // here
            NUMBER_OF_LINES,
            line,
        );
        board
    }

    pub fn move_left(&mut self) {
        if self.fit(
            self.m_brick_coord.y as i8 - 1,
            self.m_brick_coord.x as i8 - 3,
        ) {
            self.m_brick_coord.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.fit(
            self.m_brick_coord.y as i8 - 1,
            self.m_brick_coord.x as i8 - 1,
        ) {
            self.m_brick_coord.x += 1;
        }
    }

    pub fn drop(&mut self) {
        if self.fit(self.m_brick_coord.y as i8, self.m_brick_coord.x as i8 - 2) {
            self.m_brick_coord.y += 1;
        } else {
            self.fix_brick();
            self.remove_completed_lines();
            self.check_for_lost();
            self.m_brick = self.m_next_brick.clone();
            self.m_next_brick = Brick::new();
            self.m_brick_coord.x = NUMBER_OF_COLUMNS / 2;
            self.m_brick_coord.y = 0;
        }
    }

    pub fn soft_drop(&mut self) {
        for _ in 0..2 {
            self.drop();
            self.m_score += 1;
        }
    }

    pub fn hard_drop(&mut self) {
        while self.fit(self.m_brick_coord.y as i8, self.m_brick_coord.x as i8 - 2) {
            self.drop();
            self.m_score += 2;
        }
    }

    pub fn rotate(&mut self) {
        self.m_brick.rotate();
        if !self.fit(
            self.m_brick_coord.y as i8 - 1,
            self.m_brick_coord.x as i8 - 2,
        ) {
            self.m_brick.unrotate();
        }
    }

    pub fn defeated(&self) -> bool {
        self.m_lost
    }

    pub fn reset_board(&mut self) {
        for line in self.m_board.iter_mut() {
            for column in line.iter_mut() {
                *column = BoardPossibilities::Empty;
            }
        }
        self.m_score = 0;
        self.m_lines = 0;
        self.m_lost = false;
        self.m_brick = self.m_next_brick.clone();
        self.m_next_brick = Brick::new();
        self.m_brick_coord = Coord {
            x: NUMBER_OF_COLUMNS / 2,
            y: 0,
        };
    }

    pub fn consult(&self, line: usize, column: usize) -> &BoardPossibilities {
        let x = Self::distance(column as i8, self.m_brick_coord.x as i8);
        let y = Self::distance(line as i8, self.m_brick_coord.y as i8);
        if x > -3
            && x < 2
            && y > -2
            && y < 3
            && self.m_brick.consult((y + 1) as usize, (x + 2) as usize)
        {
            return self.m_brick.consult_color();
        } else {
            return &self.m_board[line][column];
        }
    }

    pub fn consult_lines_completed(&self) -> u32 {
        self.m_lines
    }

    pub fn consult_score(&self) -> u32 {
        self.m_score
    }

    pub fn display_board(&self, message: String) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line::from(message));
        for line in 0..NUMBER_OF_LINES {
            let mut spans: Vec<Span> = Vec::new();
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            for column in 0..NUMBER_OF_COLUMNS {
                match self.consult(line, column) {
                    BoardPossibilities::Red => {
                        spans.push(Span::styled("██", Style::default().fg(Color::Red)));
                    }
                    BoardPossibilities::Green => {
                        spans.push(Span::styled("██", Style::default().fg(Color::Green)));
                    }
                    BoardPossibilities::Blue => {
                        spans.push(Span::styled("██", Style::default().fg(Color::Blue)));
                    }
                    BoardPossibilities::Cyan => {
                        spans.push(Span::styled("██", Style::default().fg(Color::Cyan)));
                    }
                    BoardPossibilities::Pink => {
                        spans.push(Span::styled("██", Style::default().fg(Color::LightMagenta)));
                    }
                    BoardPossibilities::Orange => {
                        spans.push(Span::styled("██", Style::default().fg(Color::Magenta)));
                    }
                    BoardPossibilities::Yellow => {
                        spans.push(Span::styled("██", Style::default().fg(Color::Yellow)));
                    }
                    BoardPossibilities::Empty => {
                        spans.push(Span::styled("  ", Style::default()));
                    }
                }
            }
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            lines.push(Line::from(spans));
        }
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::styled("╰", Style::default().fg(Color::DarkGray)));
        for _counter in 1..NUMBER_OF_COLUMNS + 1 {
            spans.push(Span::styled("──", Style::default().fg(Color::DarkGray)));
        }
        spans.push(Span::styled("╯", Style::default().fg(Color::DarkGray)));
        lines.push(Line::from(spans));
        return lines;
    }

    pub fn display_next_brick(&self) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        for i in 0..4 {
            let mut spans: Vec<Span> = Vec::new();
            for j in 0..4 {
                if self.m_next_brick.consult(i, j) {
                    match self.m_next_brick.consult_color() {
                        BoardPossibilities::Red => {
                            spans.push(Span::styled("██", Style::default().fg(Color::Red)));
                        }
                        BoardPossibilities::Green => {
                            spans.push(Span::styled("██", Style::default().fg(Color::Green)));
                        }
                        BoardPossibilities::Blue => {
                            spans.push(Span::styled("██", Style::default().fg(Color::Blue)));
                        }
                        BoardPossibilities::Cyan => {
                            spans.push(Span::styled("██", Style::default().fg(Color::Cyan)));
                        }
                        BoardPossibilities::Pink => {
                            spans
                                .push(Span::styled("██", Style::default().fg(Color::LightMagenta)));
                        }
                        BoardPossibilities::Orange => {
                            spans.push(Span::styled("██", Style::default().fg(Color::Magenta)));
                        }
                        BoardPossibilities::Yellow => {
                            spans.push(Span::styled("██", Style::default().fg(Color::Yellow)));
                        }
                        BoardPossibilities::Empty => {}
                    }
                } else {
                    spans.push(Span::styled("  ", Style::default()));
                }
            }
            lines.push(Line::from(spans));
        }
        lines
    }

    fn fit(&self, iy: i8, ix: i8) -> bool {
        for py in 0..4 {
            for px in 0..4 {
                let x = ix + px as i8;
                let y = iy + py as i8;
                if self.m_brick.consult(py, px) {
                    if x < 0
                        || y < 0
                        || x >= NUMBER_OF_COLUMNS as i8
                        || y >= NUMBER_OF_LINES as i8
                        || !matches!(
                            self.m_board[y as usize][x as usize],
                            BoardPossibilities::Empty
                        )
                    {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn distance(origin: i8, destiny: i8) -> i8 {
        origin - destiny
    }

    fn line_completed(&self, index: usize) -> bool {
        for column in self.m_board[index].iter() {
            if matches!(*column, BoardPossibilities::Empty) {
                return false;
            }
        }
        true
    }

    fn remove_completed_lines(&mut self) {
        let mut completed = 0;
        for delta in 0..NUMBER_OF_LINES {
            let index = NUMBER_OF_LINES - delta - 1;
            if self.line_completed(index) {
                completed += 1;
            } else {
                self.m_board[index + completed] = self.m_board[index].clone();
            }
        }
        match completed {
            1 => self.m_score += 300,
            2 => self.m_score += 800,
            3 => self.m_score += 2000,
            4 => self.m_score += 5000,
            _ => (),
        }
        self.m_lines += completed as u32;
    }

    fn check_for_lost(&mut self) {
        for column in self.m_board[0].iter() {
            if !matches!(column, BoardPossibilities::Empty) {
                self.m_lost = true;
            }
        }
    }

    fn fix_brick(&mut self) {
        for py in 0..4 {
            for px in 0..4 {
                let x: i32 = self.m_brick_coord.x as i32 - 2 + px as i32;
                let y: i32 = self.m_brick_coord.y as i32 - 1 + py as i32;
                if self.m_brick.consult(py, px)
                    && x >= 0
                    && y >= 0
                    && x < NUMBER_OF_COLUMNS as i32
                    && y < NUMBER_OF_LINES as i32
                {
                    self.m_board[y as usize][x as usize] = *self.m_brick.consult_color();
                }
            }
        }
    }
}
