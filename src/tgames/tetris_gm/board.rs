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
    board: Vec<Vec<BoardPossibilities>>,
    lost: bool,
    score: u32,
    lines: u32,
    brick: Brick,
    next_brick: Brick,
    brick_coord: Coord,
}
impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            board: Vec::with_capacity(NUMBER_OF_LINES),
            lost: false,
            score: 0,
            lines: 0,
            brick: Brick::new(),
            next_brick: Brick::new(),
            brick_coord: Coord {
                x: NUMBER_OF_COLUMNS / 2,
                y: 0,
            },
        };
        let mut line: Vec<BoardPossibilities> = Vec::with_capacity(NUMBER_OF_COLUMNS);
        for _counter in 0..NUMBER_OF_COLUMNS {
            line.push(BoardPossibilities::Empty);
        }
        board.board.resize(
            // here
            NUMBER_OF_LINES,
            line,
        );
        board
    }

    pub fn move_left(&mut self) {
        if self.fit(self.brick_coord.y as i8 - 1, self.brick_coord.x as i8 - 3) {
            self.brick_coord.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.fit(self.brick_coord.y as i8 - 1, self.brick_coord.x as i8 - 1) {
            self.brick_coord.x += 1;
        }
    }

    pub fn drop(&mut self) {
        if self.fit(self.brick_coord.y as i8, self.brick_coord.x as i8 - 2) {
            self.brick_coord.y += 1;
        } else {
            self.fix_brick();
            self.remove_completed_lines();
            self.check_for_lost();
            self.brick = self.next_brick.clone();
            self.next_brick = Brick::new();
            self.brick_coord.x = NUMBER_OF_COLUMNS / 2;
            self.brick_coord.y = 0;
        }
    }

    pub fn soft_drop(&mut self) {
        for _ in 0..2 {
            self.drop();
            self.score += 1;
        }
    }

    pub fn hard_drop(&mut self) {
        while self.fit(self.brick_coord.y as i8, self.brick_coord.x as i8 - 2) {
            self.drop();
            self.score += 2;
        }
    }

    pub fn rotate(&mut self) {
        self.brick.rotate();
        if !self.fit(self.brick_coord.y as i8 - 1, self.brick_coord.x as i8 - 2) {
            self.brick.unrotate();
        }
    }

    pub fn defeated(&self) -> bool {
        self.lost
    }

    pub fn reset_board(&mut self) {
        for line in self.board.iter_mut() {
            for column in line.iter_mut() {
                *column = BoardPossibilities::Empty;
            }
        }
        self.score = 0;
        self.lines = 0;
        self.lost = false;
        self.brick = self.next_brick.clone();
        self.next_brick = Brick::new();
        self.brick_coord = Coord {
            x: NUMBER_OF_COLUMNS / 2,
            y: 0,
        };
    }

    pub fn consult(&self, line: usize, column: usize) -> &BoardPossibilities {
        let x = Self::distance(column as i8, self.brick_coord.x as i8);
        let y = Self::distance(line as i8, self.brick_coord.y as i8);
        if x > -3
            && x < 2
            && y > -2
            && y < 3
            && self.brick.consult((y + 1) as usize, (x + 2) as usize)
        {
            self.brick.consult_color()
        } else {
            &self.board[line][column]
        }
    }

    pub fn consult_lines_completed(&self) -> u32 {
        self.lines
    }

    pub fn consult_score(&self) -> u32 {
        self.score
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
        lines
    }

    pub fn display_next_brick(&self) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        for i in 0..4 {
            let mut spans: Vec<Span> = Vec::new();
            for j in 0..4 {
                if self.next_brick.consult(i, j) {
                    match self.next_brick.consult_color() {
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
                if self.brick.consult(py, px)
                    && (x < 0
                        || y < 0
                        || x >= NUMBER_OF_COLUMNS as i8
                        || y >= NUMBER_OF_LINES as i8
                        || !matches!(
                            self.board[y as usize][x as usize],
                            BoardPossibilities::Empty
                        ))
                {
                    return false;
                }
            }
        }
        true
    }

    fn distance(origin: i8, destiny: i8) -> i8 {
        origin - destiny
    }

    fn line_completed(&self, index: usize) -> bool {
        for column in self.board[index].iter() {
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
                self.board[index + completed] = self.board[index].clone();
            }
        }
        match completed {
            1 => self.score += 300,
            2 => self.score += 800,
            3 => self.score += 2000,
            4 => self.score += 5000,
            _ => (),
        }
        self.lines += completed as u32;
    }

    fn check_for_lost(&mut self) {
        for column in self.board[0].iter() {
            if !matches!(column, BoardPossibilities::Empty) {
                self.lost = true;
            }
        }
    }

    fn fix_brick(&mut self) {
        for py in 0..4 {
            for px in 0..4 {
                let x: i32 = self.brick_coord.x as i32 - 2 + px as i32;
                let y: i32 = self.brick_coord.y as i32 - 1 + py as i32;
                if self.brick.consult(py, px)
                    && x >= 0
                    && y >= 0
                    && x < NUMBER_OF_COLUMNS as i32
                    && y < NUMBER_OF_LINES as i32
                {
                    self.board[y as usize][x as usize] = *self.brick.consult_color();
                }
            }
        }
    }
}
