mod board;

use self::board::Board;

use super::game_manager::{self, Directions, GameManager};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{Result, Stdout};

enum PlayOpt {
    Direction(Directions),
    None,
    Quit,
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
    Lost,
    Quitting,
}

pub struct G2048GameManager<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    game_state: GameState,
    menu_opt: MenuOpt,
    play_opts: PlayOpt,
    record: u32,
    board: Board,
}

impl<'a> GameManager for G2048GameManager<'a> {
    fn process_events(&mut self) -> Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Helping => game_manager::read_key()?,
            GameState::Menu | GameState::Lost => self.read_menu_input()?,
            GameState::Playing => self.read_play_input()?,
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        match self.game_state {
            GameState::Starting => self.game_state = GameState::Playing,
            GameState::Helping => self.game_state = GameState::Menu,
            GameState::Menu | GameState::Lost => match self.menu_opt {
                MenuOpt::Play => {
                    if matches!(self.game_state, GameState::Lost) {
                        self.board.reset_board();
                    }
                    self.board.start_game();
                    self.game_state = GameState::Playing;
                }
                MenuOpt::Help => self.game_state = GameState::Helping,
                MenuOpt::Quit => self.game_state = GameState::Quitting,
                MenuOpt::None => (),
            },
            GameState::Playing => {
                match &self.play_opts {
                    PlayOpt::Direction(direction) => self.board.move_pieces(direction),
                    PlayOpt::Quit => {
                        self.game_state = GameState::Menu;
                        self.board.reset_board();
                    }
                    PlayOpt::None => (),
                }
                if self.board.defeated() {
                    self.game_state = GameState::Lost;
                }
                if self.record < self.board.consult_score() {
                    self.record = self.board.consult_score();
                }
            }
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Helping => self.display_game_rules()?,
            GameState::Menu => self.display_screen(
                self.record,
                G2048GameManager::menu_guide(),
                "Menu",
                "Record",
                "",
                Color::Gray,
            )?,
            GameState::Playing => self.display_screen(
                self.board.consult_score(),
                G2048GameManager::play_guide(),
                "Game board",
                "Score",
                "",
                Color::Gray,
            )?,
            GameState::Lost => self.display_screen(
                self.record,
                G2048GameManager::menu_guide(),
                "Menu",
                "Record",
                "You Lost.\nPress enter to try again.",
                Color::Red,
            )?,
            GameState::Quitting => (),
        }
        Ok(())
    }
    fn ended(&self) -> bool {
        matches!(self.game_state, GameState::Quitting)
    }
}

impl<'a> G2048GameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            terminal,
            game_state: GameState::Starting,
            menu_opt: MenuOpt::None,
            play_opts: PlayOpt::None,
            record: 0,
            board: Board::new(),
        }
    }

    fn menu_guide() -> String {
        String::from(
            "ENTER or p - Play\n?          - Display game rules\nESC or q   - Go to main menu\n",
        )
    }

    fn play_guide() -> String {
        String::from(
            "w or k or    - Move up\ns or j or    - Move down\nd or l or    - Move right\na or h or    - Move left\nESC or q      - Go to menu",
        )
    }

    fn display_screen(
        &mut self,
        score: u32,
        help_message: String,
        title: &str,
        score_title: &str,
        message: &str,
        color: Color,
    ) -> Result<()> {
        self.terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.size());
            let sub_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Fill(1),
                ])
                .split(layout[1]);

            frame.render_widget(
                Paragraph::new(self.board.display_board(message.to_string(), color)).block(
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
                Paragraph::new(self.board.consult_number_of_moves().to_string()).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Number of moves")
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[1],
            );

            frame.render_widget(
                Paragraph::new(help_message).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Help")
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[2],
            );
        })?;
        Ok(())
    }

    fn display_game_rules(&mut self) -> Result<()> {
        let message = String::from("TODO That is not done yet.\nPress any key to continue.");
        self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area);
        })?;
        Ok(())
    }

    fn read_menu_input(&mut self) -> Result<()> {
        loop {
            match read()? {
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
                    self.menu_opt = MenuOpt::Quit;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('?'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.menu_opt = MenuOpt::Help;
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
                    self.menu_opt = MenuOpt::Play;
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn read_play_input(&mut self) -> Result<()> {
        loop {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opts = PlayOpt::Direction(Directions::Left);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opts = PlayOpt::Direction(Directions::Right);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opts = PlayOpt::Direction(Directions::Down);
                    break;
                }
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
                }) => {
                    self.play_opts = PlayOpt::Direction(Directions::Up);
                    break;
                }
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
                    self.play_opts = PlayOpt::Quit;
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    }
}
