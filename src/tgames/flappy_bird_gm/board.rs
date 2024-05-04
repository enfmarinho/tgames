use rand::Rng;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::collections::LinkedList;

const NUMBER_OF_PIPES: i32 = 10; // Should not be smaller than 2
const PIPE_WIDTH: i32 = 2;
const PIPES_DISTANCE: i32 = 7;
const PIPES_HOLE_SIZE: i32 = 3;
const MAX_DISTANCE_BETWEEN_HOLES: i32 = 6;
const JUMP: i32 = 2;
const BOARD_HEIGHT: i32 = 20;
const BOARD_WIDHT: i32 = (PIPE_WIDTH + PIPES_DISTANCE) * NUMBER_OF_PIPES;

pub struct Board {
    score: u32,
    bird_height: i32,
    pipe_holes: LinkedList<i32>,
    distance_to_next_pipe: i32,
    died_horizontally: bool,
    died_vertically: bool,
    in_pipe: i32,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            score: 0,
            bird_height: BOARD_HEIGHT / 2,
            pipe_holes: LinkedList::new(),
            distance_to_next_pipe: PIPES_DISTANCE - 1,
            died_horizontally: false,
            died_vertically: false,
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
        self.died_horizontally = false;
        self.died_vertically = false;
        self.in_pipe = 0;
    }

    fn in_hole(&self) -> bool {
        let center_of_hole = self
            .pipe_holes
            .front()
            .expect("m_pipe_holes was empty when trying to advance");
        self.bird_height <= center_of_hole + PIPES_HOLE_SIZE / 2
            && self.bird_height >= center_of_hole - PIPES_HOLE_SIZE / 2
    }

    pub fn advance(&mut self, jump: bool) {
        if jump {
            self.bird_height += JUMP;
        } else {
            self.bird_height -= 1;
        }

        if self.distance_to_next_pipe == 1 && !self.in_hole() {
            self.died_horizontally = true;
        } else if self.distance_to_next_pipe == 0 && !self.in_hole() {
            self.died_vertically = true;
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
        self.died_horizontally || self.died_vertically
    }

    pub fn consult_score(&self) -> u32 {
        self.score
    }

    fn push_horizontal_board(lines: &mut Vec<Line>, up: bool) {
        let mut spans: String = String::new();
        if up {
            spans += "╭";
        } else {
            spans += "╰";
        }
        for _ in 0..BOARD_WIDHT + PIPES_DISTANCE {
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

    pub fn display_board(&self, message: String, color: Color) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        Self::push_horizontal_board(&mut lines, true);
        for line in (0..BOARD_HEIGHT).rev() {
            let mut spans: Vec<Span> = Vec::new();
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            let mut number_of_spaces = self.distance_to_next_pipe;
            if line == self.bird_height {
                if self.died_vertically || self.died_horizontally {
                    spans.push(Span::styled("󰯈", Style::default().fg(Color::Red)));
                } else {
                    spans.push(Span::styled("󱗆", Style::default().fg(Color::Blue)));
                }
                number_of_spaces -= 1;
            }
            for _ in 0..number_of_spaces {
                spans.push(Span::styled(" ", Style::default().fg(Color::Green)));
            }
            let mut first_pipe = true;
            for hole in self.pipe_holes.iter() {
                if line > hole + PIPES_HOLE_SIZE / 2 || line < hole - PIPES_HOLE_SIZE / 2 {
                    if line == self.bird_height && self.died_vertically && first_pipe {
                        spans.push(Span::styled("█", Style::default().fg(Color::Green)));
                    } else {
                        let start = if first_pipe { self.in_pipe } else { 0 };
                        for _ in start..PIPE_WIDTH {
                            spans.push(Span::styled("█", Style::default().fg(Color::Green)));
                        }
                    }
                } else {
                    for _ in 0..PIPE_WIDTH {
                        spans.push(Span::styled(" ", Style::default().fg(Color::Gray)));
                    }
                }
                for _ in 0..PIPES_DISTANCE {
                    spans.push(Span::styled(" ", Style::default().fg(Color::Gray)));
                }
                first_pipe = false;
            }
            for _ in self.distance_to_next_pipe..PIPES_DISTANCE {
                spans.push(Span::styled(" ", Style::default().fg(Color::Gray)));
            }
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            lines.push(Line::from(spans));
        }
        Self::push_horizontal_board(&mut lines, false);
        if !message.is_empty() {
            lines.push(Line::from(Span::styled(message, color)));
            lines.push(Line::from(Span::styled(
                "Press enter to continue.",
                Style::default(),
            )));
        }
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
            self.pipe_holes.push_back(BOARD_HEIGHT - PIPES_HOLE_SIZE);
        } else {
            self.pipe_holes.push_back(new_pos);
        }
    }
}
