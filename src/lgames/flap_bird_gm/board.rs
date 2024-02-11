use rand::Rng;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::collections::LinkedList;

const BOARD_HEIGHT: i32 = 30;
const PIPE_WIDTH: i32 = 6;
const PIPES_DISTANCE: i32 = 4;
const PIPES_HOLE_SIZE: i32 = 4;
const NUMBER_OF_HOLES: i32 = 15;
const MAX_DISTANCE_BETWEEN_HOLES: i32 = 5;
const JUMP: i32 = 2;

pub struct Board {
    m_score: u32,
    m_bird_height: i32,
    m_pipe_holes: LinkedList<i32>,
    m_distance_to_next_pipe: i32,
    m_died: bool,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            m_score: 0,
            m_bird_height: BOARD_HEIGHT / 2,
            m_pipe_holes: LinkedList::new(),
            m_distance_to_next_pipe: PIPES_DISTANCE - 1,
            m_died: false,
        };
        board.reset_board();
        board
    }

    pub fn reset_board(&mut self) {
        while self.m_pipe_holes.len() < NUMBER_OF_HOLES as usize {
            self.generate_pipe();
        }
        self.m_score = 0;
        self.m_bird_height = BOARD_HEIGHT / 2;
        self.m_distance_to_next_pipe = PIPES_DISTANCE;
        self.m_died = false;
    }

    pub fn advance(&mut self, jump: bool) {
        if jump {
            self.m_bird_height += JUMP;
        } else {
            self.m_bird_height -= 1;
        }
        let center_of_hole = self
            .m_pipe_holes
            .front()
            .expect("m_pipe_holes was empty when trying to advance");
        if self.m_distance_to_next_pipe == 0
            && (self.m_bird_height > center_of_hole + PIPES_HOLE_SIZE / 2
                || self.m_bird_height < center_of_hole - PIPES_HOLE_SIZE / 2)
        {
            self.m_died = true;
        } else if self.m_distance_to_next_pipe == 0 {
            self.m_distance_to_next_pipe = PIPES_DISTANCE;
            self.m_pipe_holes.pop_front();
            self.generate_pipe();
            self.m_score += 1;
        } else {
            self.m_distance_to_next_pipe -= 1;
        }
    }

    pub fn lost(&self) -> bool {
        self.m_died
    }

    pub fn consult_score(&self) -> u32 {
        self.m_score
    }

    pub fn display_board(&mut self, _message: String) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let mut spaces: String = String::new();
        let mut pipe: String = String::new();
        for _counter in 0..PIPE_WIDTH {
            spaces.push(' ');
            pipe.push('█');
        }
        for line in (0..BOARD_HEIGHT).rev() {
            let mut spans: Vec<Span> = Vec::new();
            let mut number_of_spaces = self.m_distance_to_next_pipe;
            if line == self.m_bird_height {
                match self.m_died {
                    true => spans.push(Span::styled("     󰯈", Style::default().fg(Color::Red))),
                    false => spans.push(Span::styled("     󱗆", Style::default().fg(Color::Blue))),
                }
                number_of_spaces -= 1;
            }
            for _ in 0..number_of_spaces {
                spans.push(Span::styled("  ", Style::default().fg(Color::Green)));
            }
            for hole in self.m_pipe_holes.iter() {
                // TODO check for holes
                if line > hole + PIPES_HOLE_SIZE / 2 || line < hole - PIPES_HOLE_SIZE / 2 {
                    spans.push(Span::styled(
                        pipe.clone(),
                        Style::default().fg(Color::Green),
                    ));
                } else {
                    spans.push(Span::styled(
                        spaces.clone(),
                        Style::default().fg(Color::Gray),
                    ));
                }
                for _ in 0..PIPES_DISTANCE {
                    spans.push(Span::styled(
                        spaces.clone(),
                        Style::default().fg(Color::Gray),
                    ));
                }
            }
            lines.push(Line::from(spans));
        }
        lines
    }

    fn generate_pipe(&mut self) {
        let delta =
            rand::thread_rng().gen_range(-MAX_DISTANCE_BETWEEN_HOLES..MAX_DISTANCE_BETWEEN_HOLES);
        let new_pos = match self.m_pipe_holes.back() {
            Some(value) => *value + delta,
            None => BOARD_HEIGHT / 2,
        };
        if new_pos < 0 {
            self.m_pipe_holes.push_back(0);
        } else if new_pos > BOARD_HEIGHT {
            self.m_pipe_holes.push_back(BOARD_HEIGHT - 1);
        } else {
            self.m_pipe_holes.push_back(new_pos);
        }
    }
}
