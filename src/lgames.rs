pub mod game_manager;
pub mod snake_gm;
pub mod tetris_gm;
// pub mod snaze_gm;
// pub mod sudoku_gm;

use self::{game_manager::GameManager, snake_gm::SnakeGameManager, tetris_gm::TetrisGameManager};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Paragraph},
    Terminal,
};
use std::io::{Result, Stdout};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter, FromRepr};

#[derive(EnumIter, FromRepr, Display, EnumCountMacro)]
enum Games {
    Snake,
    Tetris,
    Snaze,
    Sudoku,
}

enum MainMenuOpts {
    Play,
    Quit,
    Help,
    Up,
    Down,
    None,
}

enum LGamesState {
    Starting,
    MainMenu,
    Helping,
    Quitting,
}

pub struct LGamesManager {
    m_terminal: Terminal<CrosstermBackend<Stdout>>,
    m_execution_state: LGamesState,
    m_main_menu_opts: MainMenuOpts,
    m_game_index: usize,
}

impl GameManager for LGamesManager {
    fn process_events(&mut self) -> Result<()> {
        match self.m_execution_state {
            LGamesState::Starting => (),
            LGamesState::Helping => {
                game_manager::read_key()?;
            }
            LGamesState::MainMenu => {
                self.read_main_menu_input()?;
            }
            LGamesState::Quitting => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        match self.m_execution_state {
            LGamesState::Starting => {
                self.m_execution_state = LGamesState::MainMenu;
            }
            LGamesState::Helping => {
                self.m_execution_state = LGamesState::MainMenu;
            }
            LGamesState::MainMenu => match self.m_main_menu_opts {
                MainMenuOpts::Play => {
                    self.run_game()?;
                }
                MainMenuOpts::Help => {
                    self.m_execution_state = LGamesState::Helping;
                }
                MainMenuOpts::Quit => {
                    self.m_execution_state = LGamesState::Quitting;
                }
                MainMenuOpts::Up => {
                    if self.m_game_index > 0 {
                        self.m_game_index -= 1;
                    }
                }
                MainMenuOpts::Down => {
                    if self.m_game_index < Games::COUNT - 1 {
                        self.m_game_index += 1;
                    }
                }
                MainMenuOpts::None => (),
            },
            LGamesState::Quitting => (),
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        match self.m_execution_state {
            LGamesState::Starting => (),
            LGamesState::Helping => {
                self.display_help_message()?;
            }
            LGamesState::MainMenu => {
                self.display_main_menu()?;
            }
            LGamesState::Quitting => (),
        }
        Ok(())
    }

    fn ended(&self) -> bool {
        matches!(self.m_execution_state, LGamesState::Quitting)
    }
}

impl LGamesManager {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>) -> LGamesManager {
        LGamesManager {
            m_terminal: terminal,
            m_execution_state: LGamesState::Starting,
            m_main_menu_opts: MainMenuOpts::None,
            m_game_index: 0,
        }
    }

    fn display_help_message(&mut self) -> Result<()> {
        let message = String::from("This is still not done.\nPress any key to continue.");
        self.m_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area);
        })?;
        Ok(())
    }

    fn display_main_menu(&mut self) -> Result<()> {
        let mut lines: Vec<Line> = Vec::new();
        for (index, opts) in Games::iter().enumerate() {
            if index == self.m_game_index {
                lines.push(Line::from(Span::styled(
                    "> ".to_string() + &opts.to_string() + &" <".to_string(),
                    Style::default().green(),
                )));
            } else {
                lines.push(Line::from(
                    Span::styled(opts.to_string(), Style::default()).gray(),
                ));
            }
            lines.push(Line::from(Span::styled("\n\n", Style::default()).gray()));
        }
        self.m_terminal.draw(|frame| {
            let block = Block::default()
                .title(Title::from("LGames").alignment(Alignment::Center))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded);
            frame.render_widget(Paragraph::new(lines).block(block), frame.size());
        })?;
        Ok(())
    }

    fn read_main_menu_input(&mut self) -> Result<()> {
        loop {
            let event = event::read()?;
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    self.m_main_menu_opts = MainMenuOpts::Quit;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    self.m_main_menu_opts = MainMenuOpts::Play;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    self.m_main_menu_opts = MainMenuOpts::Up;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    self.m_main_menu_opts = MainMenuOpts::Down;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('?'),
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    self.m_main_menu_opts = MainMenuOpts::Help;
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    }
    fn run_game(&mut self) -> Result<()> {
        let option_game = Games::from_repr(self.m_game_index);
        let game: Games;
        match option_game {
            Some(value) => {
                game = value;
            }
            None => {
                // Should not reach this.
                self.m_game_index = 0;
                return Ok(());
            }
        }
        match game {
            Games::Snake => {
                SnakeGameManager::new(&mut self.m_terminal).run()?;
            }
            Games::Tetris => {
                TetrisGameManager::new(&mut self.m_terminal).run()?;
            }
            Games::Snaze => {
                // TODO delete this.
                let message = String::from("snaze was selected");
                self.m_terminal.draw(|frame| {
                    let area = frame.size();
                    frame.render_widget(Paragraph::new(message).white(), area);
                })?;
                game_manager::read_key()?;
            }
            Games::Sudoku => {
                // TODO delete this.
                let message = String::from("sudoku was selected");
                self.m_terminal.draw(|frame| {
                    let area = frame.size();
                    frame.render_widget(Paragraph::new(message).white(), area);
                })?;
                game_manager::read_key()?;
            }
        }
        Ok(())
    }
}
