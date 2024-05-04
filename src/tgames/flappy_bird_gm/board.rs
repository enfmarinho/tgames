use rand::Rng;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::collections::LinkedList;

const BOARD_HEIGHT: i32 = 20;
const NUMBER_OF_PIPES: i32 = 5; // Should not be smaller than 2
const PIPE_WIDTH: i32 = 2;
const PIPES_DISTANCE: i32 = 8;
const PIPES_HOLE_SIZE: i32 = 4;
const MAX_DISTANCE_BETWEEN_HOLES: i32 = 5;
const JUMP: i32 = 2;

pub struct Board {
    score: u32,
    bird_height: i32,
    pipe_holes: LinkedList<i32>,
    distance_to_next_pipe: i32,
    died: bool,
    in_pipe: i32,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            score: 0,
            bird_height: BOARD_HEIGHT / 2,
            pipe_holes: LinkedList::new(),
            distance_to_next_pipe: PIPES_DISTANCE - 1,
            died: false,
            in_pipe: 0,
        };
        board.reset_board();
        board
    }

    pub fn reset_board(&mut self) {
        self.pipe_holes.clear();
        self.pipe_holes.push_front(BOARD_HEIGHT / 2);
        while self.pipe_holes.len() < NUMBER_OF_PIPES as usize {
            self.generate_pipe();
        }
        self.score = 0;
        self.bird_height = BOARD_HEIGHT / 2;
        self.distance_to_next_pipe = PIPES_DISTANCE;
        self.died = false;
    }

    pub fn advance(&mut self, jump: bool) {
        if jump {
            self.bird_height += JUMP;
        } else {
            self.bird_height -= 1;
        }
        let center_of_hole = self
            .pipe_holes
            .front()
            .expect("m_pipe_holes was empty when trying to advance");
        if (self.distance_to_next_pipe == 1 || self.distance_to_next_pipe == 0)
            && (self.bird_height > center_of_hole + PIPES_HOLE_SIZE / 2
                || self.bird_height < center_of_hole - PIPES_HOLE_SIZE / 2)
        {
            self.died = true;
        } else if self.distance_to_next_pipe == 0 && self.in_pipe == PIPE_WIDTH - 1 {
            self.distance_to_next_pipe = PIPES_DISTANCE;
            self.in_pipe = 0;
            self.score += 1;
            self.pipe_holes.pop_front();
            self.generate_pipe();
        } else if self.distance_to_next_pipe == 0 {
            self.in_pipe += 1;
        } else {
            self.distance_to_next_pipe -= 1;
        }
    }

    pub fn lost(&self) -> bool {
        self.died
    }

    pub fn consult_score(&self) -> u32 {
        self.score
    }

    // TODO adapt this function.
    pub fn display_board(&mut self, _message: String) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let mut spans: String = String::new();
        spans += "╭";
        for _ in 1..20 {
            spans += "──";
        }
        spans += "╮";
        lines.push(Line::from(Span::styled(
            spans,
            Style::default().fg(Color::DarkGray),
        )));
        let mut pipe: String = String::new();
        for _ in 0..PIPE_WIDTH {
            pipe.push('█');
        }
        for line in (0..BOARD_HEIGHT).rev() {
            let mut spans: Vec<Span> = Vec::new();
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            let mut number_of_spaces = self.distance_to_next_pipe;
            if line == self.bird_height {
                match self.died {
                    true => spans.push(Span::styled("󰯈", Style::default().fg(Color::Red))),
                    false => spans.push(Span::styled("󱗆", Style::default().fg(Color::Blue))),
                }
                number_of_spaces -= 1;
            }
            for _ in 0..number_of_spaces {
                spans.push(Span::styled(" ", Style::default().fg(Color::Green)));
            }
            for hole in self.pipe_holes.iter() {
                if line > hole + PIPES_HOLE_SIZE / 2 || line < hole - PIPES_HOLE_SIZE / 2 {
                    spans.push(Span::styled(
                        pipe.clone(),
                        Style::default().fg(Color::Green),
                    ));
                } else {
                    spans.push(Span::styled(" ", Style::default().fg(Color::Gray)));
                }
                for _ in 0..PIPES_DISTANCE {
                    spans.push(Span::styled(" ", Style::default().fg(Color::Gray)));
                }
            }
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            lines.push(Line::from(spans));
        }
        let mut spans: String = String::new();
        spans += "╰";
        for _ in 1..20 {
            spans += "──";
        }
        spans += "╯";
        lines.push(Line::from(Span::styled(
            spans,
            Style::default().fg(Color::DarkGray),
        )));
        lines
    }

    fn generate_pipe(&mut self) {
        let delta =
            rand::thread_rng().gen_range(-MAX_DISTANCE_BETWEEN_HOLES..MAX_DISTANCE_BETWEEN_HOLES);
        let new_pos = match self.pipe_holes.back() {
            Some(value) => *value + delta,
            None => BOARD_HEIGHT / 2,
        };
        if new_pos < 0 {
            self.pipe_holes.push_back(PIPES_HOLE_SIZE);
        } else if new_pos > BOARD_HEIGHT {
            self.pipe_holes.push_back(BOARD_HEIGHT - 1);
        } else {
            self.pipe_holes.push_back(new_pos);
        }
    }
}
