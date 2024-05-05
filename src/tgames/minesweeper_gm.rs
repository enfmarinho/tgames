mod board;

use super::game_manager::{self, read_key, Difficult, Directions};
use board::Board;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{Result, Stdout};

enum MenuOpt {
    Play(Difficult),
    Quit,
    Help,
    None,
}

#[derive(PartialEq)]
enum GameState {
    Starting,
    Menu,
    Playing,
    Helping,
    Won,
    Lost,
    Quitting,
}

enum PlayOpt {
    Reveal,
    Mark,
    Direction(Directions),
    Quit,
    None,
}

pub struct MinesweeperGameManager<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    game_state: GameState,
    play_opt: PlayOpt,
    menu_opt: MenuOpt,
    board: Board,
    record: usize,
}

impl<'a> game_manager::GameManager for MinesweeperGameManager<'a> {
    fn process_events(&mut self) -> std::io::Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Menu | GameState::Won | GameState::Lost => self.read_menu_input()?,
            GameState::Playing => self.read_play_input()?,
            GameState::Helping => read_key()?,
            GameState::Quitting => (),
        }
        Ok(())
    }
    fn update(&mut self) -> std::io::Result<()> {
        match self.game_state {
            GameState::Starting => self.game_state = GameState::Playing,
            GameState::Helping => self.game_state = GameState::Menu,
            GameState::Menu | GameState::Won | GameState::Lost => match &self.menu_opt {
                MenuOpt::Play(difficult) => {
                    self.game_state = GameState::Playing;
                    self.board.reset(difficult);
                }
                MenuOpt::Help => self.game_state = GameState::Helping,
                MenuOpt::Quit => self.game_state = GameState::Quitting,
                MenuOpt::None => (),
            },
            GameState::Playing => match &self.play_opt {
                PlayOpt::Reveal => {
                    self.board.reveal();
                    if self.board.won() {
                        self.game_state = GameState::Won;
                    } else if self.board.lost() {
                        self.game_state = GameState::Lost;
                    }
                    if self.board.score() > self.record {
                        self.record = self.board.score();
                    }
                }
                PlayOpt::Mark => self.board.mark(),
                PlayOpt::Direction(direction) => self.board.move_cursor(direction),
                PlayOpt::Quit => self.game_state = GameState::Menu,
                PlayOpt::None => (),
            },
            GameState::Quitting => {}
        }
        Ok(())
    }
    fn render(&mut self) -> std::io::Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Menu => self.display_screen(
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "",
                Color::default(),
            )?,
            GameState::Playing => self.display_screen(
                self.board.score(),
                Self::play_guide(),
                "Board",
                "Score",
                "",
                Color::default(),
            )?,
            GameState::Helping => self.display_game_rules()?,
            GameState::Won => self.display_screen(
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You won, congratulations!",
                Color::Green,
            )?,
            GameState::Lost => self.display_screen(
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You lost, try again!",
                Color::Green,
            )?,
            GameState::Quitting => (),
        }
        Ok(())
    }
    fn ended(&self) -> bool {
        self.game_state == GameState::Quitting
    }
}

impl<'a> MinesweeperGameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            terminal,
            game_state: GameState::Starting,
            play_opt: PlayOpt::None,
            menu_opt: MenuOpt::None,
            board: Board::new(),
            record: 0,
        }
    }

    fn menu_guide() -> String {
        String::from("TODO")
    }

    fn play_guide() -> String {
        String::from("TODO")
    }

    fn display_screen(
        &mut self,
        score: usize,
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
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
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
        let message = String::from(
"Picture a grid, like a little field, filled with hidden mines and numbers. Your job is to
uncover all the squares on the grid without detonating any mines.

You start by clicking on any square to reveal what's underneath. If you're lucky, you might
uncover an empty square, which shows you the number of neighboring squares that contain mines.

The numbers are your clues. They tell you how many mines are adjacent to that square. So, if
you see a \"1\" next to a square, it means there's one mine nearby. A \"2\" means two mines,
and so on.

But here's the catch: if you click on a square and it reveals a mine, boom! Game over. So, you
have to use logic and deduction to figure out where the mines are hidden and safely mark them 
with flags.

You can mark a square as a potential mine by pressing 'm'. This helps you keep track of where
you think the mines might be.

The game ends when you've uncovered all the squares that don't contain mines. If you manage to 
clear the entire grid without detonating any mines, you win!

So, in summary: click to uncover squares, use the numbers to avoid the mines, and mark potential
mines with flags. It's a classic game of strategy and deduction that's perfect for relaxing and 
exercising your brain!");
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
                }) => {
                    self.menu_opt = MenuOpt::Play(Difficult::Medium);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.menu_opt = MenuOpt::Play(Difficult::Easy);
                    break;
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char('m'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.menu_opt = MenuOpt::Play(Difficult::Medium);
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.menu_opt = MenuOpt::Play(Difficult::Hard);
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
                    code: KeyCode::Char('m'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('!'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Mark;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.play_opt = PlayOpt::Reveal;
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
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    }
}
