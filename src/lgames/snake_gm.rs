mod board;

use super::game_manager::{self, Directions};
use board::Board;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};
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
            GameState::Menu => {
                self.read_menu_input()?;
            }
            GameState::Helping => {
                game_manager::read_key()?;
            }
            GameState::Playing => {
                self.read_play_input()?;
            }
            GameState::Won => {
                game_manager::read_key()?;
            }
            GameState::Lost => {
                game_manager::read_key()?;
            }
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => {
                self.m_game_state = GameState::Menu;
            }
            GameState::Helping => {
                self.m_game_state = GameState::Menu;
            }
            GameState::Menu => match self.m_menu_opt {
                MenuOpt::Play => {
                    self.m_game_state = GameState::Playing;
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
                    self.m_game_state = GameState::Menu;
                }
                self.m_board.move_snake(&self.m_direction);
                if self.m_board.snake_died() {
                    self.m_game_state = GameState::Lost;
                } else if self.m_board.won() {
                    self.m_game_state = GameState::Won;
                }
            }
            GameState::Won => {
                if self.m_record < self.m_board.consult_score() {
                    self.m_record = self.m_board.consult_score();
                }
                self.m_game_state = GameState::Menu;
                self.m_direction = Directions::Right;
                self.m_board.reset_board();
            }
            GameState::Lost => {
                if self.m_record < self.m_board.consult_score() {
                    self.m_record = self.m_board.consult_score();
                }
                self.m_game_state = GameState::Menu;
                self.m_direction = Directions::Right;
                self.m_board.reset_board();
            }
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => (),
            GameState::Menu => self.display_menu_screen()?,
            GameState::Helping => self.display_help()?,
            GameState::Playing => self.display_game_screen()?,
            GameState::Won => self.display_victory_screen()?,
            GameState::Lost => self.display_defeat_screen()?,
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

    fn display_help(&mut self) -> Result<()> {
        let message = String::from("There are only two rules you must follow when playing: don’t hit a wall and don’t bite your own tail.\nYou can move the snake using the arrows keys or the vim keys.\nYou win the game when there is no more room for your snake to grow.\nYour high score is calculated based on the number of squares you added to the snake.");
        self.m_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
    }

    fn display_menu_screen(&mut self) -> Result<()> {
        let message =
            String::from("This is the menu.\nPress enter to play\nPress f to change fps.\nScore: ")
                + &self.m_record.to_string();
        self.m_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
    }

    fn display_game_screen(&mut self) -> Result<()> {
        let message =
            self.m_board.display_board() + "Score: " + &self.m_board.consult_score().to_string();
        self.m_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
    }

    fn display_victory_screen(&mut self) -> Result<()> {
        let message = String::from("You won, congratulations!!\n") + &self.m_board.display_board();
        self.m_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
    }

    fn display_defeat_screen(&mut self) -> Result<()> {
        let message = String::from("You lost, try again!\n") + &self.m_board.display_board();
        self.m_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
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
