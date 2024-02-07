mod board;
mod bricks;

use super::game_manager::{self, GameManager};
use board::Board;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io::{Result, Stdout},
    thread::sleep,
    time::{Duration, SystemTime},
};

enum MenuOpt {
    Play,
    Quit,
    Help,
    None,
}

enum PlayOpt {
    Left,
    Right,
    Rotate,
    SoftDrop,
    HardDrop,
    Pause,
    Quit,
    None,
}

enum GameState {
    Starting,
    Helping,
    Menu,
    Playing,
    Pause,
    Lost,
    Quitting,
}

pub struct TetrisGameManager<'a> {
    m_terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    m_game_state: GameState,
    m_menu_opt: MenuOpt,
    m_play_opt: PlayOpt,
    m_board: Board,
    m_counter: usize,
    m_score_record: u32,
    m_line_record: u32,
}
impl<'a> GameManager for TetrisGameManager<'a> {
    fn process_events(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => (),
            GameState::Helping => game_manager::read_key()?,
            GameState::Menu | GameState::Lost => self.read_menu_input()?,
            GameState::Playing => self.read_play_input()?,
            GameState::Pause => game_manager::read_key()?,
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => self.m_game_state = GameState::Playing,
            GameState::Helping => self.m_game_state = GameState::Menu,
            GameState::Menu | GameState::Lost => match self.m_menu_opt {
                MenuOpt::Play => {
                    self.m_board.reset_board();
                    self.m_game_state = GameState::Playing;
                }
                MenuOpt::Help => {
                    self.m_game_state = GameState::Helping;
                }
                MenuOpt::Quit => {
                    self.m_game_state = GameState::Quitting;
                }
                MenuOpt::None => (),
            },
            GameState::Playing => {
                self.m_counter += 1;
                if self.m_counter > 3 {
                    self.m_board.drop();
                    self.m_counter = 0;
                }
                match self.m_play_opt {
                    PlayOpt::Left => {
                        self.m_board.move_left();
                    }
                    PlayOpt::Right => {
                        self.m_board.move_right();
                    }
                    PlayOpt::Pause => {
                        self.m_game_state = GameState::Pause;
                    }
                    PlayOpt::Rotate => {
                        self.m_board.rotate();
                    }
                    PlayOpt::SoftDrop => {
                        self.m_board.soft_drop();
                        self.m_counter = 0;
                    }
                    PlayOpt::HardDrop => {
                        self.m_board.hard_drop();
                        self.m_counter = 0;
                    }
                    PlayOpt::Quit => {
                        self.m_game_state = GameState::Menu;
                        if self.m_board.consult_score() > self.m_score_record {
                            self.m_score_record = self.m_board.consult_score();
                        }
                        if self.m_board.consult_lines_completed() > self.m_line_record {
                            self.m_line_record = self.m_board.consult_lines_completed();
                        }
                    }
                    PlayOpt::None => (),
                }
                if self.m_board.defeated() {
                    self.m_game_state = GameState::Lost;
                    if self.m_board.consult_score() > self.m_score_record {
                        self.m_score_record = self.m_board.consult_score();
                    }
                    if self.m_board.consult_lines_completed() > self.m_line_record {
                        self.m_line_record = self.m_board.consult_lines_completed();
                    }
                }
            }
            GameState::Pause => self.m_game_state = GameState::Playing,
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => (),
            GameState::Helping => self.display_game_rules()?,
            GameState::Menu => self.display_screen(
                self.m_score_record,
                self.m_line_record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "",
            )?,
            GameState::Playing => self.display_screen(
                self.m_board.consult_score(),
                self.m_board.consult_lines_completed(),
                Self::play_guide(),
                "Game board",
                "Score",
                "",
            )?,
            GameState::Pause => self.display_screen(
                self.m_score_record,
                self.m_line_record,
                Self::menu_guide(),
                "Menu",
                "Score",
                "Game is paused.\nPress enter to continue.",
            )?,
            GameState::Lost => self.display_screen(
                self.m_score_record,
                self.m_line_record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You lost!\nPress enter to try again.",
            )?,
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn ended(&self) -> bool {
        matches!(self.m_game_state, GameState::Quitting)
    }
}

impl<'a> TetrisGameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            m_terminal: terminal,
            m_game_state: GameState::Starting,
            m_menu_opt: MenuOpt::None,
            m_play_opt: PlayOpt::None,
            m_board: Board::new(),
            m_counter: 0,
            m_score_record: 0,
            m_line_record: 0,
        }
    }

    fn menu_guide() -> String {
        String::from("") // TODO write message
    }

    fn play_guide() -> String {
        String::from("") // TODO write message
    }

    fn display_screen(
        &mut self,
        score: u32,
        line_score: u32,
        help_message: String,
        title: &str,
        score_title: &str,
        message: &str,
    ) -> Result<()> {
        self.m_terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.size());
            let sub_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Fill(1),
                ])
                .split(layout[1]);

            frame.render_widget(
                Paragraph::new(self.m_board.display_board(message)).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title(title)
                        .title_alignment(Alignment::Center),
                ),
                layout[0],
            );

            frame.render_widget(
                Paragraph::new(self.m_board.display_next_brick()).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Next brick")
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[0],
            );

            frame.render_widget(
                Paragraph::new(score.to_string()).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Game ".to_string() + score_title)
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[1],
            );

            frame.render_widget(
                Paragraph::new(line_score.to_string()).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Line ".to_string() + score_title)
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[2],
            );

            frame.render_widget(
                Paragraph::new(help_message).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Help")
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[3],
            );
        })?;
        Ok(())
    }

    fn display_game_rules(&mut self) -> Result<()> {
        let message = String::from("TODO write game rules.");
        self.m_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
    }

    fn read_menu_input(&mut self) -> Result<()> {
        loop {
            let event = read()?;
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.m_menu_opt = MenuOpt::Quit;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('?'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.m_menu_opt = MenuOpt::Help;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.m_menu_opt = MenuOpt::Play;
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn read_play_input(&mut self) -> Result<()> {
        let time = SystemTime::now();
        let wait_time = Duration::from_millis(50);
        if poll(wait_time)? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.m_play_opt = PlayOpt::Left,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.m_play_opt = PlayOpt::Right,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.m_play_opt = PlayOpt::SoftDrop,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.m_play_opt = PlayOpt::Rotate,
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.m_play_opt = PlayOpt::HardDrop,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.m_play_opt = PlayOpt::Pause,
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.m_play_opt = PlayOpt::Quit;
                }
                _ => (),
            }
        } else {
            self.m_play_opt = PlayOpt::None;
        }
        match time.elapsed() {
            Ok(duration) => {
                let sleep_time: u128;
                // sleep for the difference of
                if duration.as_millis() > wait_time.as_millis() {
                    sleep_time = duration.as_millis() - wait_time.as_millis();
                } else {
                    sleep_time = wait_time.as_millis() - duration.as_millis();
                }
                sleep(Duration::from_millis(sleep_time as u64));
            }
            Err(_) => (),
        }
        Ok(())
    }
}
