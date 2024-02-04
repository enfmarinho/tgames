mod board;

use super::game_manager::{self, Directions};
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
    time::Duration,
};

const FPS_CHANGE: u64 = 15;

enum MenuOpt {
    Play,
    Quit,
    Help,
    None,
    IncreaseFPS,
    DecreaseFPS,
}

enum GameState {
    Starting,
    Menu,
    Playing,
    Helping,
    Won,
    Lost,
    Quitting,
}

pub struct SnakeGameManager<'a> {
    m_terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    m_game_state: GameState,
    m_menu_opt: MenuOpt,
    m_direction: Directions,
    m_board: Board,
    m_record: u32,
    m_fps: u64,
}
impl<'a> game_manager::GameManager for SnakeGameManager<'a> {
    fn process_events(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => (),
            GameState::Menu | GameState::Won | GameState::Lost => {
                self.read_menu_input()?;
            }
            GameState::Helping => {
                game_manager::read_key()?;
            }
            GameState::Playing => {
                self.read_play_input()?;
            }
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => {
                self.m_game_state = GameState::Playing;
            }
            GameState::Helping => {
                self.m_game_state = GameState::Menu;
            }
            GameState::Menu | GameState::Won | GameState::Lost => match self.m_menu_opt {
                MenuOpt::Play => {
                    self.m_game_state = GameState::Playing;
                    if self.m_record < self.m_board.consult_score() {
                        self.m_record = self.m_board.consult_score();
                    }
                    self.m_direction = Directions::Right;
                    self.m_board.reset_board();
                }
                MenuOpt::Quit => {
                    self.m_game_state = GameState::Quitting;
                }
                MenuOpt::Help => {
                    self.m_game_state = GameState::Helping;
                }
                MenuOpt::IncreaseFPS => {
                    self.change_fps(self.m_fps + FPS_CHANGE);
                }
                MenuOpt::DecreaseFPS => {
                    self.change_fps(self.m_fps - FPS_CHANGE);
                }
                MenuOpt::None => {}
            },
            GameState::Playing => {
                if matches!(self.m_menu_opt, MenuOpt::Quit) {
                    self.m_board.reset_board();
                    self.m_game_state = GameState::Menu;
                }
                self.m_board.move_snake(&self.m_direction);
                let mut ended: bool = false;
                if self.m_board.snake_died() {
                    self.m_game_state = GameState::Lost;
                    ended = true;
                } else if self.m_board.won() {
                    self.m_game_state = GameState::Won;
                    ended = true;
                }
                if ended && self.m_record < self.m_board.consult_score() {
                    self.m_record = self.m_board.consult_score();
                }
            }
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => (),
            GameState::Menu => self.display_screen()?,
            GameState::Helping => self.display_rules()?,
            GameState::Playing => self.display_screen()?,
            GameState::Won => self.display_screen()?,
            GameState::Lost => self.display_screen()?,
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn ended(&self) -> bool {
        matches!(self.m_game_state, GameState::Quitting)
    }

    fn limit_fps(&self) {
        std::thread::sleep(std::time::Duration::from_millis(1000 / self.m_fps));
    }
}
impl<'a> SnakeGameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        SnakeGameManager {
            m_terminal: terminal,
            m_game_state: GameState::Starting,
            m_menu_opt: MenuOpt::None,
            m_direction: Directions::Right,
            m_board: Board::new(12, 20),
            m_record: 0,
            m_fps: 20,
        }
    }

    fn display_rules(&mut self) -> Result<()> {
        let message = String::from("There are only two rules you must follow when playing: don’t hit a wall and don’t bite your own tail.\nYou can move the snake using the arrows keys or the vim keys.\nYou win the game when there is no more room for your snake to grow.\nYour high score is calculated based on the number of squares you added to the snake.");
        self.m_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
    }

    fn display_screen(&mut self) -> Result<()> {
        let score: u32;
        let help_message: String;
        let title: String;
        let score_title: String;
        let mut message: String = String::new();
        if matches!(self.m_game_state, GameState::Playing) {
            score = self.m_board.consult_score();
            help_message = Self::play_guide();
            title = String::from("Game board");
            score_title = String::from("Score");
        } else {
            score = self.m_record;
            help_message = Self::menu_guide();
            title = String::from("Menu");
            score_title = String::from("Record");
        }
        if matches!(self.m_game_state, GameState::Won) {
            message = String::from("You won, congratulations!\nPress enter to play again.\n");
        } else if matches!(self.m_game_state, GameState::Lost) {
            message = String::from("You lost!\nPress enter to try again.\n");
        }

        self.m_terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.size());
            let sub_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
                .split(layout[1]);

            frame.render_widget(
                Paragraph::new(message + &self.m_board.display_board()).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title(title)
                        .title_alignment(Alignment::Center),
                ),
                layout[0],
            );

            frame.render_widget(
                Paragraph::new(score.to_string()).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title(score_title)
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[0],
            );

            frame.render_widget(
                Paragraph::new(help_message).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Help")
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[1],
            );
        })?;
        Ok(())
    }

    fn play_guide() -> String {
        String::from(
            "'k' or    - Move up\n'j' or    - Move down\n'l' or    - Move right\n'h' or    - Move left\n'q' or ESC - Go to menu",
        )
    }

    fn menu_guide() -> String {
        String::from("")
    }

    fn read_play_input(&mut self) -> Result<()> {
        if poll(Duration::from_millis(100))? {
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
                }) => self.m_direction = Directions::Left,
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
                }) => self.m_direction = Directions::Right,
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
                }) => self.m_direction = Directions::Down,
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
                }) => self.m_direction = Directions::Up,
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
                    self.m_menu_opt = MenuOpt::Quit;
                }
                _ => (),
            }
        }
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
                Event::Key(KeyEvent {
                    code: KeyCode::Char('f'),
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    self.m_menu_opt = MenuOpt::DecreaseFPS;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('f'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.m_menu_opt = MenuOpt::IncreaseFPS;
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn change_fps(&mut self, fps: u64) {
        self.m_fps = fps;
    }
}
