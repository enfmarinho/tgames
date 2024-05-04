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
    board: Vec<BoardPossibilities>,
    height: usize,
    width: usize,
    score: u32,
    snake: VecDeque<Coord>,
}
impl Board {
    pub fn new(board_height: usize, board_width: usize) -> Self {
        let mut board = Self {
            board: Vec::with_capacity(board_height * board_width),
            height: board_height,
            width: board_width,
            score: 1,
            snake: VecDeque::new(),
        };
        board
            .board
            .resize(board.height * board.width, BoardPossibilities::Empty);
        board.reset_board();
        board
    }

    pub fn snake_died(&self) -> bool {
        match self.snake.front() {
            Some(coord) => matches!(
                self.consult_board(coord.y, coord.x),
                BoardPossibilities::SnakeDead
            ),
            None => false,
        }
    }
    pub fn won(&self) -> bool {
        self.score == (self.height * self.width) as u32
    }

    pub fn move_snake(&mut self, direction: &Directions) {
        let head: Coord;
        if let Some(value) = self.snake.front() {
            head = value.clone();
        } else {
            panic!();
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
            self.score += 1;
            self.generate_food();
        } else if let Some(value) = self.snake.pop_back() {
            *self.get_board_position(&value.y, &value.x) = BoardPossibilities::Empty;
        }
        *self.get_board_position(&new_head_pos.y, &new_head_pos.x) = BoardPossibilities::SnakeHead;
        self.snake.push_front(new_head_pos);
    }

    pub fn consult_score(&self) -> u32 {
        self.score
    }

    pub fn display_board(&self, message: String, color: Color) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let mut spans: String = String::new();
        spans += "╭";
        for _counter in 1..self.width + 1 {
            spans += "──";
        }
        spans += "╮";
        lines.push(Line::from(Span::styled(
            spans,
            Style::default().fg(Color::DarkGray),
        )));
        for i in 0..self.height {
            let mut spans: Vec<Span> = Vec::new();
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            for j in 0..self.width {
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
        for _counter in 1..self.width + 1 {
            spans += "──";
        }
        spans += "╯";
        lines.push(Line::from(
            Span::styled(spans, Style::default()).fg(Color::DarkGray),
        ));
        if !message.is_empty() {
            lines.push(Line::from(Span::styled(message, color)));
            lines.push(Line::from(Span::styled(
                "Press enter to play again.",
                Style::default(),
            )));
        }
        lines
    }

    pub fn reset_board(&mut self) {
        self.score = 1;
        self.board.fill(BoardPossibilities::Empty);
        self.snake.clear();
        self.snake.push_front(Coord {
            x: self.width / 3,
            y: self.height / 2,
        });
        self.board[self.height / 2 * self.width + self.width - (self.width / 3)] =
            BoardPossibilities::Food;
        self.board[self.height / 2 * self.width + self.width / 3] = BoardPossibilities::SnakeHead;
    }

    fn generate_food(&mut self) {
        let mut line = rand::thread_rng().gen_range(0..self.height);
        let mut column = rand::thread_rng().gen_range(0..self.width);
        while !matches!(self.consult_board(line, column), BoardPossibilities::Empty) {
            column += 1;
            if column == self.width {
                column = 0;
                line += 1;
            }
            if line == self.height {
                line = 0;
            }
        }
        *self.get_board_position(&line, &column) = BoardPossibilities::Food;
    }

    fn consult_board(&self, line: usize, column: usize) -> &BoardPossibilities {
        &self.board[line * self.width + column]
    }

    fn get_board_position(&mut self, line: &usize, column: &usize) -> &mut BoardPossibilities {
        &mut self.board[line * self.width + column]
    }

    fn position_valid(&self, coord: &Coord) -> bool {
        coord.x > 0 && coord.x <= self.width && coord.y > 0 && coord.y <= self.height
    }
}
