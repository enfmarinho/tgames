mod board;
mod bricks;

use super::game_manager::{
    self, should_force_quit, should_help, should_move_down, should_move_left, should_move_right,
    should_move_up, should_pause, should_play, should_quit, GameManager,
};
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

#[derive(PartialEq, Eq)]
enum GameState {
    Starting,
    Helping,
    Menu,
    Playing,
    Pause,
    Lost,
    AskingToQuit,
    Quitting,
}

pub struct TetrisGameManager<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    game_state: GameState,
    menu_opt: MenuOpt,
    play_opt: PlayOpt,
    confirmed: bool,
    board: Board,
    counter: usize,
    score_record: u32,
    line_record: u32,
    kill_execution: bool,
}
impl<'a> GameManager for TetrisGameManager<'a> {
    fn process_events(&mut self) -> Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Helping => game_manager::read_key()?,
            GameState::Menu | GameState::Lost => self.read_menu_input()?,
            GameState::Playing => self.read_play_input()?,
            GameState::Pause => game_manager::read_key()?,
            GameState::AskingToQuit => {
                let event = read()?;
                self.kill_execution = game_manager::should_force_quit(&event);
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
            GameState::Helping => self.game_state = GameState::Menu,
            GameState::Menu | GameState::Lost => match self.menu_opt {
                MenuOpt::Play => {
                    if let GameState::Lost = self.game_state {
                        self.board.reset_board();
                    }
                    self.game_state = GameState::Playing;
                }
                MenuOpt::Help => {
                    self.game_state = GameState::Helping;
                }
                MenuOpt::Quit => {
                    self.game_state = GameState::Quitting;
                }
                MenuOpt::None => (),
            },
            GameState::Playing => {
                self.counter += 1;
                if self.counter > 3 {
                    self.board.drop();
                    self.counter = 0;
                }
                match self.play_opt {
                    PlayOpt::Left => {
                        self.board.move_left();
                    }
                    PlayOpt::Right => {
                        self.board.move_right();
                    }
                    PlayOpt::Pause => {
                        self.game_state = GameState::Pause;
                    }
                    PlayOpt::Rotate => {
                        self.board.rotate();
                    }
                    PlayOpt::SoftDrop => {
                        self.board.soft_drop();
                        self.counter = 0;
                    }
                    PlayOpt::HardDrop => {
                        self.board.hard_drop();
                        self.counter = 0;
                    }
                    PlayOpt::Quit => {
                        self.game_state = GameState::AskingToQuit;
                        self.update_record();
                    }
                    PlayOpt::None => (),
                }
                if self.board.defeated() {
                    self.game_state = GameState::Lost;
                    self.update_record();
                }
            }
            GameState::Pause => self.game_state = GameState::Playing,
            GameState::AskingToQuit => match self.confirmed {
                true => {
                    self.game_state = GameState::Menu;
                    self.board.reset_board();
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
                self.score_record,
                self.line_record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "",
            )?,
            GameState::Playing => self.display_screen(
                self.board.consult_score(),
                self.board.consult_lines_completed(),
                Self::play_guide(),
                "Game board",
                "Score",
                "",
            )?,
            GameState::Pause => self.display_screen(
                self.score_record,
                self.line_record,
                Self::menu_guide(),
                "Menu",
                "Score",
                "Game is paused. Press enter to continue.",
            )?,
            GameState::Lost => self.display_screen(
                self.score_record,
                self.line_record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You lost! Press enter to try again.",
            )?,
            GameState::AskingToQuit => self.display_screen(
                self.board.consult_score(),
                self.board.consult_lines_completed(),
                game_manager::confirmation_guide(),
                "Quitting",
                "Score",
                "Are you sure you want to quit?",
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
}

impl<'a> TetrisGameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            terminal,
            game_state: GameState::Starting,
            menu_opt: MenuOpt::None,
            play_opt: PlayOpt::None,
            confirmed: false,
            board: Board::new(),
            counter: 0,
            score_record: 0,
            line_record: 0,
            kill_execution: false,
        }
    }

    fn menu_guide() -> String {
        String::from(
            "ENTER or p - Play\n?          - Display game rules\nESC or q   - Go to main menu\n",
        )
    }

    fn play_guide() -> String {
        String::from(
            "d or l or  - Move piece to the right\na or h or  - Move piece to the left\nw or k or  - Rotate piece\ns or j or  - Soft drop\nSPACE       - Hard drop\np           - Pause game\nESC or q    - Go back to menu\n ",
        )
    }

    fn update_record(&mut self) {
        if self.board.consult_score() > self.score_record {
            self.score_record = self.board.consult_score();
        }
        if self.board.consult_lines_completed() > self.line_record {
            self.line_record = self.board.consult_lines_completed();
        }
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
                    Constraint::Percentage(15),
                    Constraint::Fill(1),
                ])
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
                Paragraph::new(self.board.display_next_brick()).block(
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
        let message = String::from(
            "Tetris is like a puzzle game where you fit different shapes together to clear lines.
Imagine you have a little playground, and colorful blocks start falling from the sky,
one at a time. These blocks come in different shapes, like squares, long sticks, L's,
and zigzags.

Your job is to move and rotate these blocks as they fall, trying to fit them neatly at
the bottom. You can slide them left or right and even spin them around until they lock
into place. When you create a solid line across the playground with no gaps, that line
disappears, making room for more blocks.

But here's the trick: the blocks keep falling as you play, so you need to think and act
fast to keep up. If the blocks pile up and reach the top of the playground, it's game 
over!

So, in short: move, spin, and stack the falling blocks to make solid lines and keep the 
playground clear. It's easy to learn, but oh-so-addictive once you get going!",
        );
        self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area)
        })?;
        Ok(())
    }

    fn read_menu_input(&mut self) -> Result<()> {
        loop {
            let event = read()?;
            if should_quit(&event) {
                self.menu_opt = MenuOpt::Quit;
                break;
            } else if should_force_quit(&event) {
                self.kill_execution = true;
                break;
            } else if should_play(&event) {
                self.menu_opt = MenuOpt::Play;
                break;
            } else if should_help(&event) {
                self.menu_opt = MenuOpt::Help;
                break;
            }
        }
        Ok(())
    }

    fn read_play_input(&mut self) -> Result<()> {
        if poll(Duration::from_millis(50))? {
            let event = read()?;
            if should_move_left(&event) {
                self.play_opt = PlayOpt::Left;
            } else if should_move_right(&event) {
                self.play_opt = PlayOpt::Right;
            } else if should_move_down(&event) {
                self.play_opt = PlayOpt::SoftDrop;
            } else if should_move_up(&event) {
                self.play_opt = PlayOpt::Rotate;
            } else if should_pause(&event) {
                self.play_opt = PlayOpt::Pause;
            } else if should_quit(&event) {
                self.play_opt = PlayOpt::Quit;
            } else if should_force_quit(&event) {
                self.kill_execution = true;
            } else if matches!(
                event,
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
            ) {
                self.play_opt = PlayOpt::HardDrop;
            }
        } else {
            self.play_opt = PlayOpt::None;
        }
        Ok(())
    }
}
