use super::game_manager::{Coord, Directions};
use rand::Rng;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use std::collections::VecDeque;
use std::vec::Vec;

#[derive(Clone)]
enum BoardPossibilities {
    SnakeBody,
    SnakeHead,
    SnakeDead,
    Food,
    Empty,
}

pub struct Board {
    m_board: Vec<BoardPossibilities>,
    m_height: usize,
    m_width: usize,
    m_score: u32,
    m_snake: VecDeque<Coord>,
}
impl Board {
    pub fn new(board_height: usize, board_width: usize) -> Self {
        let mut board = Self {
            m_board: Vec::with_capacity(board_height * board_width),
            m_height: board_height,
            m_width: board_width,
            m_score: 1,
            m_snake: VecDeque::new(),
        };
        board
            .m_board
            .resize(board.m_height * board.m_width, BoardPossibilities::Empty);
        board.reset_board();
        return board;
    }

    pub fn snake_died(&self) -> bool {
        match self.m_snake.front() {
            Some(coord) => matches!(
                self.consult_board(coord.y, coord.x),
                BoardPossibilities::SnakeDead
            ),
            None => false,
        }
    }
    pub fn won(&self) -> bool {
        self.m_score == (self.m_height * self.m_width) as u32
    }

    pub fn move_snake(&mut self, direction: &Directions) {
        let head: Coord;
        match self.m_snake.front() {
            Some(value) => head = value.clone(),
            None => panic!(),
        }
        let mut new_head_pos = head.clone();
        // making indexing from one to avoid overflow, since its unsigned.
        new_head_pos.x += 1;
        new_head_pos.y += 1;
        match direction {
            Directions::Up => new_head_pos.y -= 1,
            Directions::Down => new_head_pos.y += 1,
            Directions::Left => new_head_pos.x -= 1,
            Directions::Right => new_head_pos.x += 1,
        }
        if !self.position_valid(&new_head_pos) {
            *self.get_board_position(&head.y, &head.x) = BoardPossibilities::SnakeDead;
            return;
        }
        new_head_pos.x -= 1;
        new_head_pos.y -= 1;
        *self.get_board_position(&head.y, &head.x) = BoardPossibilities::SnakeBody;
        if matches!(
            self.consult_board(new_head_pos.y, new_head_pos.x),
            BoardPossibilities::SnakeBody
        ) {
            *self.get_board_position(&head.y, &head.x) = BoardPossibilities::SnakeDead;
            return;
        } else if matches!(
            self.consult_board(new_head_pos.y, new_head_pos.x),
            BoardPossibilities::Food
        ) {
            self.m_score += 1;
            self.generate_food();
        } else {
            match self.m_snake.pop_back() {
                Some(value) => {
                    *self.get_board_position(&value.y, &value.x) = BoardPossibilities::Empty;
                }
                None => (),
            }
        }
        *self.get_board_position(&new_head_pos.y, &new_head_pos.x) = BoardPossibilities::SnakeHead;
        self.m_snake.push_front(new_head_pos);
    }

    pub fn consult_score(&self) -> u32 {
        self.m_score
    }

    pub fn display_board(&self, message: String, color: Color) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        if message != "" {
            lines.push(Line::from(Span::styled(message, color)));
            lines.push(Line::from(Span::styled(
                "Press enter to play again.",
                Style::default(),
            )));
        }
        let mut spans: String = String::new();
        spans += "╭";
        for _counter in 1..self.m_width + 1 {
            spans += "──";
        }
        spans += "╮";
        lines.push(Line::from(Span::styled(
            spans,
            Style::default().fg(Color::DarkGray),
        )));
        for i in 0..self.m_height {
            let mut spans: Vec<Span> = Vec::new();
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            for j in 0..self.m_width {
                match self.consult_board(i, j) {
                    BoardPossibilities::Empty => spans.push(Span::styled("  ", Style::default())),
                    BoardPossibilities::SnakeBody => {
                        spans.push(Span::styled("██", Style::default().fg(Color::Blue)));
                    }
                    BoardPossibilities::SnakeHead => {
                        spans.push(Span::styled("██", Style::default().fg(Color::Cyan)));
                    }
                    BoardPossibilities::SnakeDead => {
                        spans.push(Span::styled("󰯈 ", Style::default().fg(Color::Red)));
                    }
                    BoardPossibilities::Food => {
                        spans.push(Span::styled("󰉛 ", Style::default().fg(Color::LightRed)));
                    }
                }
            }
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            lines.push(Line::from(spans));
        }
        let mut spans: String = String::new();
        spans += "╰";
        for _counter in 1..self.m_width + 1 {
            spans += "──";
        }
        spans += "╯";
        lines.push(Line::from(
            Span::styled(spans, Style::default()).fg(Color::DarkGray),
        ));
        lines
    }

    pub fn reset_board(&mut self) {
        self.m_score = 1;
        self.m_board.fill(BoardPossibilities::Empty);
        self.m_snake.clear();
        self.m_snake.push_front(Coord {
            x: self.m_width / 3,
            y: self.m_height / 2,
        });
        self.m_board[self.m_height / 2 * self.m_width + self.m_width - (self.m_width / 3)] =
            BoardPossibilities::Food;
        self.m_board[self.m_height / 2 * self.m_width + self.m_width / 3] =
            BoardPossibilities::SnakeHead;
    }

    fn generate_food(&mut self) {
        let mut line = rand::thread_rng().gen_range(0..self.m_height);
        let mut column = rand::thread_rng().gen_range(0..self.m_width);
        while !matches!(self.consult_board(line, column), BoardPossibilities::Empty) {
            column += 1;
            if column == self.m_width {
                column = 0;
                line += 1;
            }
            if line == self.m_height {
                line = 0;
            }
        }
        *self.get_board_position(&line, &column) = BoardPossibilities::Food;
    }

    fn consult_board(&self, line: usize, column: usize) -> &BoardPossibilities {
        &self.m_board[line * self.m_width + column]
    }

    fn get_board_position(&mut self, line: &usize, column: &usize) -> &mut BoardPossibilities {
        &mut self.m_board[line * self.m_width + column]
    }

    fn position_valid(&self, coord: &Coord) -> bool {
        coord.x > 0 && coord.x <= self.m_width && coord.y > 0 && coord.y <= self.m_height
    }
}
