mod board;

use super::{
    super::input::{
        read_confirmation, read_key, should_force_quit, should_help, should_move_down,
        should_move_left, should_move_right, should_move_up, should_play, should_quit,
    },
    game_manager::{self, Difficult, Directions},
};
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

enum PlayOpt {
    Reveal,
    Mark,
    Direction(Directions),
    Quit,
    None,
}

pub struct MinesweeperGameManager {
    game_state: GameState,
    menu_opt: MenuOpt,
    play_opt: PlayOpt,
    confirmed: bool,
    difficult: Difficult,
    board: Board,
    record: u32,
    kill_execution: bool,
}

impl game_manager::GameManager for MinesweeperGameManager {
    fn process_events(&mut self) -> Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Menu | GameState::Won | GameState::Lost => self.read_menu_input()?,
            GameState::Playing => self.read_play_input()?,
            GameState::Helping => read_key()?,
            GameState::AskingToQuit => {
                let event = read()?;
                self.kill_execution = should_force_quit(&event);
                self.confirmed = read_confirmation(&event);
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
                PlayOpt::Quit => self.game_state = GameState::AskingToQuit,
                PlayOpt::None => (),
            },
            GameState::AskingToQuit => match self.confirmed {
                true => {
                    self.board.clear();
                    self.game_state = GameState::Menu;
                }
                false => {
                    self.game_state = GameState::Playing;
                }
            },
            GameState::Quitting => (),
        }
        Ok(())
    }
    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Menu => self.display_screen(
                terminal,
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "",
                Color::default(),
            )?,
            GameState::Playing => self.display_screen(
                terminal,
                self.board.score(),
                Self::play_guide(),
                "Board",
                "Score",
                "",
                Color::default(),
            )?,
            GameState::Helping => self.display_game_rules(terminal)?,
            GameState::Won => self.display_screen(
                terminal,
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You won, congratulations!",
                Color::Green,
            )?,
            GameState::Lost => self.display_screen(
                terminal,
                self.record,
                Self::menu_guide(),
                "Menu",
                "Record",
                "You lost, try again!",
                Color::Red,
            )?,
            GameState::AskingToQuit => self.display_screen(
                terminal,
                self.board.score(),
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

    fn reset(&mut self) {
        self.game_state = GameState::Starting;
        self.board.reset(&self.difficult);
    }

    fn ended(&self) -> bool {
        self.game_state == GameState::Quitting
    }

    fn kill_execution(&self) -> bool {
        self.kill_execution
    }
}

impl MinesweeperGameManager {
    pub fn new() -> Self {
        Self {
            game_state: GameState::Starting,
            menu_opt: MenuOpt::None,
            play_opt: PlayOpt::None,
            confirmed: false,
            difficult: Difficult::Medium,
            board: Board::new(),
            record: 0,
            kill_execution: false,
        }
    }

    fn menu_guide() -> String {
        String::from(
            "ENTER    - Play last difficult played\ne        - Play easy game\nm        - Play medium game\nh        - Play hard game\n?        - Display game rules\nESC or q - Go to main menu\n",
        )
    }

    fn play_guide() -> String {
        String::from(
            "ENTER or r    - Reveal square\n! or m        - Mark square\nw or k or    - Move up\ns or j or    - Move down\nd or l or    - Move right\na or h or    - Move left\nESC or q      - Go to menu",
        )
    }

    fn display_screen(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        score: u32,
        help_message: String,
        title: &str,
        score_title: &str,
        message: &str,
        color: Color,
    ) -> Result<()> {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(frame.size());
            let sub_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(70),
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
                Paragraph::new(self.board.bombs().to_string()).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Bombs remaining")
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

    fn display_game_rules(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<()> {
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
        terminal.draw(|frame| {
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
            } else if should_help(&event) {
                self.menu_opt = MenuOpt::Help;
                break;
            } else if should_play(&event) {
                self.menu_opt = MenuOpt::Play(self.difficult.clone());
                break;
            } else if matches!(
                event,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('e'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
            ) {
                self.difficult = Difficult::Easy;
                self.menu_opt = MenuOpt::Play(self.difficult.clone());
                break;
            } else if matches!(
                event,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('m'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
            ) {
                self.difficult = Difficult::Medium;
                self.menu_opt = MenuOpt::Play(self.difficult.clone());
                break;
            } else if matches!(
                event,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
            ) {
                self.difficult = Difficult::Hard;
                self.menu_opt = MenuOpt::Play(self.difficult.clone());
                break;
            }
        }
        Ok(())
    }

    fn read_play_input(&mut self) -> Result<()> {
        loop {
            let event = read()?;
            if should_move_up(&event) {
                self.play_opt = PlayOpt::Direction(Directions::Up);
                break;
            } else if should_move_down(&event) {
                self.play_opt = PlayOpt::Direction(Directions::Down);
                break;
            } else if should_move_left(&event) {
                self.play_opt = PlayOpt::Direction(Directions::Left);
                break;
            } else if should_move_right(&event) {
                self.play_opt = PlayOpt::Direction(Directions::Right);
                break;
            } else if should_quit(&event) {
                self.play_opt = PlayOpt::Quit;
                break;
            } else if should_force_quit(&event) {
                self.kill_execution = true;
                break;
            } else if matches!(
                event,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('m'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) | Event::Key(KeyEvent {
                    code: KeyCode::Char('!'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
            ) {
                self.play_opt = PlayOpt::Mark;
                break;
            } else if matches!(
                event,
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) | Event::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })
            ) {
                self.play_opt = PlayOpt::Reveal;
                break;
            }
        }
        Ok(())
    }
}
