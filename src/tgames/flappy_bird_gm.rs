mod board;

use super::game_manager::{self, GameManager};
use board::Board;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Color,
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io::{Result, Stdout},
    time::Duration,
};

const FPS_CHANGE: u64 = 4;

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
    IncreaseFPS,
    DecreaseFPS,
    None,
}

#[derive(PartialEq, Eq)]
enum GameState {
    Starting,
    Menu,
    Playing,
    Helping,
    Pause,
    Lost,
    AskingToQuit,
    Quitting,
}

pub struct FlappyBirdGameManager<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    game_state: GameState,
    menu_opt: MenuOpt,
    play_opt: PlayOpt,
    confirmed: bool,
    board: Board,
    record: u32,
    fps: u64,
    kill_execution: bool,
}

impl<'a> GameManager for FlappyBirdGameManager<'a> {
    fn process_events(&mut self) -> Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Menu | GameState::Lost => self.read_menu_input()?,
            GameState::Playing => self.read_play_input()?,
            GameState::Helping | GameState::Pause => game_manager::read_key()?,
            GameState::AskingToQuit => {
                let event = read()?;
                self.kill_execution = game_manager::force_quit(&event);
                self.confirmed = game_manager::read_confirmation(&event);
            }
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
            GameState::Menu | GameState::Lost => match self.menu_opt {
                MenuOpt::Play => {
                    if matches!(self.game_state, GameState::Lost) {
                        self.board.reset_board();
                    }
                    self.game_state = GameState::Playing;
                }
                MenuOpt::Help => self.game_state = GameState::Helping,
                MenuOpt::IncreaseFPS => self.increase_fps(),
                MenuOpt::DecreaseFPS => self.decrease_fps(),
                MenuOpt::Quit => self.game_state = GameState::Quitting,
                MenuOpt::None => (),
            },
            GameState::Playing => {
                match self.play_opt {
                    PlayOpt::None => self.board.advance(false),
                    PlayOpt::Jump => self.board.advance(true),
                    PlayOpt::Pause => self.game_state = GameState::Pause,
                    PlayOpt::Quit => self.game_state = GameState::AskingToQuit,
                }
                if self.board.lost() {
                    self.game_state = GameState::Lost;
                }
                if self.record < self.board.consult_score() {
                    self.record = self.board.consult_score();
                }
            }
            GameState::Helping => self.game_state = GameState::Menu,
            GameState::Pause => self.game_state = GameState::Playing,
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
            GameState::Menu => self.display_screen(
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "",
                Color::default(),
            )?,
            GameState::Playing => self.display_screen(
                self.board.consult_score(),
                Self::play_guide(),
                "Game board",
                "Score",
                "",
                Color::default(),
            )?,
            GameState::Lost => self.display_screen(
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You lost!",
                Color::Red,
            )?,
            GameState::Helping => self.display_game_rules()?,
            GameState::Pause => self.display_screen(
                self.board.consult_score(),
                Self::play_guide(),
                "Game board",
                "Score",
                "Game is paused.",
                Color::default(),
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
        matches!(self.game_state, GameState::Quitting)
    }

    fn limit_fps(&self) {
        std::thread::sleep(Duration::from_millis(1000 / self.fps));
    }
}

impl<'a> FlappyBirdGameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            terminal,
            game_state: GameState::Starting,
            menu_opt: MenuOpt::None,
            play_opt: PlayOpt::None,
            confirmed: false,
            board: Board::new(),
            record: 0,
            fps: FPS_CHANGE * 2,
            kill_execution: false,
        }
    }

    fn display_game_rules(&mut self) -> Result<()> {
        let message = String::from(
            "Imagine you're a little bird trying to navigate through a series of pipes. The game 
starts with you flying in the sky, and with each jump, you flap your wings and ascend 
a bit.

Your goal is to fly as far as you can without crashing into any of the pipes. These 
pipes have openings that you need to pass through, kind of like flying through a maze.
Each successful pass scores you a point.

But here's the tricky part: the pipes are spaced at different heights, and they're always
moving towards you. So, you have to time your flaps just right to avoid hitting them.

If you touch a pipe or crash into the ground, it's game over, and you'll have to start
again from the beginning. The challenge lies in mastering the timing of your flaps to 
navigate through the narrow gaps between the pipes.

So, to sum it up: flap your wings to fly, avoid the pipes, and see how far you can go. 
It's a simple yet surprisingly addictive game that'll keep you entertained for hours!",
        );
        self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
    }

    fn menu_guide() -> String {
        String::from(
            "ENTER or p - Play\nf          - Decrease fps\nF          - Increase fps\n?          - Display game rules\nESC or q   - Go to main menu\n",
        )
    }

    fn play_guide() -> String {
        String::from("w or k or  or SPACE - Jump\np                    - Pause game\nESC or q             - Go back to menu")
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
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Fill(1)])
                .split(frame.size());
            let sub_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Fill(1)])
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
                    modifiers: KeyModifiers::SHIFT,
                    kind: KeyEventKind::Press,
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
                }) => self.play_opt = PlayOpt::Jump,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.play_opt = PlayOpt::Pause,
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
                }) => self.play_opt = PlayOpt::Quit,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }) => self.kill_execution = true,
                _ => (),
            }
        } else {
            self.play_opt = PlayOpt::None;
        }
        Ok(())
    }

    fn increase_fps(&mut self) {
        self.fps += FPS_CHANGE;
    }

    fn decrease_fps(&mut self) {
        self.fps -= FPS_CHANGE;
        if self.fps < FPS_CHANGE {
            self.fps = FPS_CHANGE;
        }
    }
}
