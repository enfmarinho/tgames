mod board;

use super::game_manager::{self, Directions};
use board::Board;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io::{Result, Stdout},
    time::Duration,
};

const FPS_CHANGE: i64 = 15;

enum MenuOpt {
    Play,
    Quit,
    Help,
    None,
    IncreaseFPS,
    DecreaseFPS,
}

#[derive(PartialEq, Eq)]
enum GameState {
    Starting,
    Menu,
    Playing,
    Helping,
    Won,
    Lost,
    AskingToQuit,
    Quitting,
}

pub struct SnakeGameManager<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    game_state: GameState,
    menu_opt: MenuOpt,
    direction: Directions,
    confirmed: bool,
    board: Board,
    record: u32,
    fps: u64,
    kill_execution: bool,
}
impl<'a> game_manager::GameManager for SnakeGameManager<'a> {
    fn process_events(&mut self) -> Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Menu | GameState::Won | GameState::Lost => self.read_menu_input()?,
            GameState::Helping => game_manager::read_key()?,
            GameState::Playing => self.read_play_input()?,
            GameState::AskingToQuit => self.confirmed = game_manager::read_confirmation(),
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        if self.kill_execution {
            self.game_state = GameState::Quitting;
        }
        match self.game_state {
            GameState::Starting => self.game_state = GameState::Playing,
            GameState::Helping => self.game_state = GameState::Menu,
            GameState::Menu | GameState::Won | GameState::Lost => match self.menu_opt {
                MenuOpt::Play => {
                    self.game_state = GameState::Playing;
                    self.direction = Directions::Right;
                    self.board.reset_board();
                }
                MenuOpt::Quit => self.game_state = GameState::Quitting,
                MenuOpt::Help => self.game_state = GameState::Helping,
                MenuOpt::IncreaseFPS => self.change_fps(self.fps as i64 + FPS_CHANGE),
                MenuOpt::DecreaseFPS => self.change_fps(self.fps as i64 - FPS_CHANGE),
                MenuOpt::None => (),
            },
            GameState::Playing => {
                self.board.move_snake(&self.direction);
                if self.board.snake_died() {
                    self.game_state = GameState::Lost;
                } else if self.board.won() {
                    self.game_state = GameState::Won;
                }
                if self.record < self.board.consult_score() {
                    self.record = self.board.consult_score();
                }
                if matches!(self.menu_opt, MenuOpt::Quit) {
                    self.game_state = GameState::AskingToQuit;
                    self.menu_opt = MenuOpt::None;
                }
            }
            GameState::AskingToQuit => match self.confirmed {
                true => {
                    self.board.reset_board();
                    self.game_state = GameState::Menu;
                }
                false => self.game_state = GameState::Playing,
            },
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
                Self::menu_guide(),
                "Menu",
                "Record",
                "",
                Color::Gray,
            )?,
            GameState::Playing => self.display_screen(
                self.board.consult_score(),
                Self::play_guide(),
                "Game board",
                "Score",
                "",
                Color::Gray,
            )?,
            GameState::Won => self.display_screen(
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You won, congratulations!!",
                Color::Green,
            )?,
            GameState::Lost => self.display_screen(
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You lost.",
                Color::Red,
            )?,
            GameState::AskingToQuit => self.display_screen(
                self.board.consult_score(),
                game_manager::confirmation_guide(),
                "Quitting",
                "Score",
                "Are you sure you want to quit?",
                Color::Yellow,
            )?,
            GameState::Quitting => (),
        }
        Ok(())
    }

    fn kill_execution(&self) -> bool {
        self.kill_execution
    }

    fn ended(&self) -> bool {
        self.game_state == GameState::Quitting
    }

    fn limit_fps(&self) {
        std::thread::sleep(Duration::from_millis(1000 / self.fps));
    }
}
impl<'a> SnakeGameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        SnakeGameManager {
            terminal,
            game_state: GameState::Starting,
            menu_opt: MenuOpt::None,
            direction: Directions::Right,
            confirmed: false,
            board: Board::new(12, 20),
            record: 0,
            fps: 15,
            kill_execution: false,
        }
    }

    fn display_game_rules(&mut self) -> Result<()> {
        let message = String::from(
            "Imagine you're a little snake, slithering around a tiny world, looking for food. In 
this game, you control the snake's movements, guiding it across the screen.

Your goal is simple: eat as much food as you can without crashing into anything. Food
appears randomly on the screen. When your snake munches on the food, it grows longer.

But here's the catch: your snake is always moving forward, and you can only control its
direction, not its speed. As your snake gets longer, maneuvering becomes trickier because
you need to avoid running into your own tail or hitting the walls.

The longer your snake grows, the more points you earn. But be careful not to get too greedy
and trap yourself!

So, to sum it up: guide your snake around, gobble up the food to grow longer, and avoid
crashing into anything. It's a simple concept, but you'll find yourself hooked in no time,
trying to beat your high score with each game!",
        );
        self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
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

    fn play_guide() -> String {
        String::from(
            "w or k or    - Move up\ns or j or    - Move down\nd or l or    - Move right\na or h or    - Move left\nESC or q      - Go to menu",
        )
    }

    fn menu_guide() -> String {
        String::from("ENTER or p - Play\nf          - Decrease fps\nF          - Increase fps\n?          - Display game rules\nESC or q   - Go to main menu\n",
        )
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
                Event::Key(KeyEvent {
                    code: KeyCode::Char('F'),
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::SHIFT,
                    ..
                }) => {
                    self.menu_opt = MenuOpt::IncreaseFPS;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('f'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.menu_opt = MenuOpt::DecreaseFPS;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.kill_execution = true;
                    break;
                }
                _ => (),
            }
        }
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
                }) => self.direction = Directions::Left,
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
                }) => self.direction = Directions::Right,
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
                }) => self.direction = Directions::Down,
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
                }) => self.direction = Directions::Up,
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
                }) => self.menu_opt = MenuOpt::Quit,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.kill_execution = true,
                _ => (),
            }
        }
        Ok(())
    }

    fn change_fps(&mut self, fps: i64) {
        if fps > 0 {
            self.fps = fps as u64;
        }
    }
}
