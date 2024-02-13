mod board;

use self::board::Board;

use super::game_manager::{self, Directions, GameManager};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{Result, Stdout};

enum PlayOpt {
    Direction(Directions),
    Place(u8),
    Remove,
    Undo,
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
    Helping,
    Playing,
    Lost,
    Won,
    Quitting,
}

pub struct SudokuGameManager<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    game_state: GameState,
    menu_opt: MenuOpt,
    play_opt: PlayOpt,
    board: Board,
    message: String,
    record: u8,
}
impl<'a> GameManager for SudokuGameManager<'a> {
    fn process_events(&mut self) -> std::io::Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Menu | GameState::Won | GameState::Lost => self.read_menu_input()?,
            GameState::Helping => game_manager::read_key()?,
            GameState::Playing => self.read_play_input()?,
            GameState::Quitting => (),
        }
        Ok(())
    }
    fn update(&mut self) -> std::io::Result<()> {
        match self.game_state {
            GameState::Starting => {
                self.game_state = GameState::Playing;
            }
            GameState::Menu | GameState::Won | GameState::Lost => match self.menu_opt {
                MenuOpt::Play => {
                    if matches!(self.game_state, GameState::Lost)
                        || matches!(self.game_state, GameState::Won)
                    {
                        self.board.reset_board();
                    }
                    self.game_state = GameState::Playing;
                }
                MenuOpt::Quit => self.game_state = GameState::Quitting,
                MenuOpt::Help => self.game_state = GameState::Helping,
                MenuOpt::None => (),
            },
            GameState::Helping => {
                self.game_state = GameState::Menu;
            }
            GameState::Playing => {
                match &self.play_opt {
                    PlayOpt::Direction(direction) => {
                        self.board.move_cursor(direction);
                    }
                    PlayOpt::Place(value) => {
                        if self.board.place(*value) {
                            self.message = "Value placed".to_string();
                        } else {
                            self.message = "Unable to place a number here".to_string();
                        }
                    }
                    PlayOpt::Remove => {
                        if self.board.remove() {
                            self.message = "Value removed.".to_string();
                        } else {
                            self.message = "Unable to remove this.".to_string();
                        }
                    }
                    PlayOpt::Undo => {
                        if self.board.undo() {
                            self.message = "Undo made".to_string();
                        } else {
                            self.message = "There is no move to undo.".to_string();
                        }
                    }
                    PlayOpt::Quit => self.game_state = GameState::Menu,
                    PlayOpt::None => (),
                }
                if self.board.lost() {
                    self.game_state = GameState::Lost;
                } else if self.board.won() {
                    self.game_state = GameState::Won;
                }
                if self.record < self.board.consult_score() {
                    self.record = self.board.consult_score();
                }
            }
            GameState::Quitting => (),
        }
        Ok(())
    }
    fn render(&mut self) -> std::io::Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Menu => {
                self.display_screen(self.record, Self::menu_guide(), "Menu", "Record,", "")?;
            }
            GameState::Helping => {
                self.display_game_rules()?;
            }
            GameState::Playing => {
                self.display_screen(
                    self.board.consult_score(),
                    Self::play_guide(),
                    "Game board",
                    "Score",
                    "",
                )?;
            }
            GameState::Won => {
                self.display_screen(
                    self.record,
                    Self::menu_guide(),
                    "Menu",
                    "Record,",
                    "You won, congratulations!\n Press enter to play again\n",
                )?;
            }
            GameState::Lost => {
                self.display_screen(
                    self.record,
                    Self::menu_guide(),
                    "Menu",
                    "Record,",
                    "You lost.\nPress enter to try again\n",
                )?;
            }
            GameState::Quitting => (),
        }
        Ok(())
    }
    fn ended(&self) -> bool {
        matches!(self.game_state, GameState::Quitting)
    }
}

impl<'a> SudokuGameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            terminal,
            game_state: GameState::Starting,
            menu_opt: MenuOpt::None,
            play_opt: PlayOpt::None,
            board: Board::new(),
            message: String::new(),
            record: 0,
        }
    }

    fn menu_guide() -> String {
        String::from(
            "ENTER or p - Play\n?          - Display game rules\nESC or q   - Go to main menu\n",
        )
    }

    fn play_guide() -> String {
        String::from("w or k or    - Move up\ns or j or    - Move down\nd or l or    - Move right\na or h or    - Move left\nu             - undo last action\nENTER or r - remove value under the cursor\n1-9 - Place value where cursor is\nESC or q      - Go to menu",)
    }

    fn display_screen(
        &mut self,
        score: u8,
        help_message: String,
        title: &str,
        score_title: &str,
        message: &str,
    ) -> Result<()> {
        self.terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.size());
            let sub_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(15), Constraint::Fill(1)])
                .split(layout[1]);

            frame.render_widget(
                Paragraph::new(self.board.display_board(message.to_string())).block(
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
                        .title("Game ".to_string() + score_title)
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

    fn display_game_rules(&mut self) -> Result<()> {
        let message = String::from("TODO write game rules.");
        self.terminal.draw(|frame| {
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
                    self.play_opt = PlayOpt::Direction(Directions::Left);
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
                    self.play_opt = PlayOpt::Direction(Directions::Right);
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
                    self.play_opt = PlayOpt::Direction(Directions::Down);
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
                    self.play_opt = PlayOpt::Direction(Directions::Up);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('u'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Undo;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Remove;
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
                    self.play_opt = PlayOpt::Quit;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('1'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Place(1);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('2'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Place(2);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('3'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Place(3);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('4'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Place(4);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('5'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Place(5);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('6'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Place(6);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('7'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Place(7);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('8'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Place(8);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('9'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Place(9);
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    }
}
