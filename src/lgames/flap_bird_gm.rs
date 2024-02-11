mod board;

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
    time::Duration,
};

const FPS_CHANGE: u64 = 2;

enum PlayOpt {
    Jump,
    Pause,
    Quit,
    None,
}

enum MenuOpt {
    Play,
    Quit,
    Help,
    None,
}

enum GameState {
    Starting,
    Menu,
    Playing,
    Helping,
    Pause,
    Lost,
    Quitting,
}

pub struct FlapBirdGameManager<'a> {
    m_terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    m_game_state: GameState,
    m_menu_opt: MenuOpt,
    m_play_opt: PlayOpt,
    m_board: Board,
    m_record: u32,
    m_fps: u64,
}

impl<'a> GameManager for FlapBirdGameManager<'a> {
    fn process_events(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => (),
            GameState::Menu | GameState::Lost => self.read_menu_input()?,
            GameState::Playing => self.read_play_input()?,
            GameState::Helping | GameState::Pause => game_manager::read_key()?,
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => self.m_game_state = GameState::Playing,
            GameState::Menu | GameState::Lost => match self.m_menu_opt {
                MenuOpt::Play => {
                    if matches!(self.m_game_state, GameState::Lost) {
                        self.m_board.reset_board();
                    }
                    self.m_game_state = GameState::Playing;
                }
                MenuOpt::Help => self.m_game_state = GameState::Helping,
                MenuOpt::Quit => self.m_game_state = GameState::Quitting,
                MenuOpt::None => (),
            },
            GameState::Playing => {
                match self.m_play_opt {
                    PlayOpt::None => self.m_board.advance(false),
                    PlayOpt::Jump => self.m_board.advance(true),
                    PlayOpt::Pause => self.m_game_state = GameState::Pause,
                    PlayOpt::Quit => self.m_game_state = GameState::Menu,
                }
                if self.m_board.lost() {
                    self.m_game_state = GameState::Lost;
                }
            }
            GameState::Helping => self.m_game_state = GameState::Menu,
            GameState::Pause => self.m_game_state = GameState::Playing,
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        match self.m_game_state {
            GameState::Starting => (),
            GameState::Menu => {
                self.display_screen(self.m_record, Self::menu_guide(), "Menu", "Record", "")?
            }
            GameState::Playing => self.display_screen(
                self.m_board.consult_score(),
                Self::play_guide(),
                "Game board",
                "Score",
                "",
            )?,
            GameState::Lost => self.display_screen(
                self.m_record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You lost!\nPress enter to try again.",
            )?,
            GameState::Helping => self.diplay_game_rules()?,
            GameState::Pause => self.display_screen(
                self.m_board.consult_score(),
                Self::play_guide(),
                "Game board",
                "Score",
                "Game is paused.\nPress enter to continue.",
            )?,
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

impl<'a> FlapBirdGameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            m_terminal: terminal,
            m_game_state: GameState::Starting,
            m_menu_opt: MenuOpt::None,
            m_play_opt: PlayOpt::None,
            m_board: Board::new(),
            m_record: 0,
            m_fps: 5,
        }
    }

    fn diplay_game_rules(&mut self) -> Result<()> {
        let message = String::from("TODO write game rules.");
        self.m_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
    }

    fn menu_guide() -> String {
        String::from(
            "ENTER or p - Play\n?          - Display game rules\nESC or q   - Go to main menu\n",
        )
    }

    fn play_guide() -> String {
        String::from("w or k or  or SPACE - Jump\np                     - Pause game\nESC or q              - Go back to menu")
    }

    fn display_screen(
        &mut self,
        score: u32,
        help_message: String,
        title: &str,
        score_title: &str,
        message: &str,
    ) -> Result<()> {
        self.m_terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(60), Constraint::Fill(1)])
                .split(frame.size());
            let sub_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Fill(1)])
                .split(layout[1]);

            frame.render_widget(
                Paragraph::new(self.m_board.display_board(message.to_string())).block(
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
        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.m_play_opt = PlayOpt::Jump,
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
        Ok(())
    }
}
