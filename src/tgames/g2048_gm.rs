mod board;

use self::board::Board;

use super::game_manager::{
    self, should_force_quit, should_help, should_move_down, should_move_left, should_move_right,
    should_move_up, should_play, should_quit, Directions, GameManager,
};
use crossterm::event::read;
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

#[derive(PartialEq, Eq)]
enum GameState {
    Starting,
    Menu,
    Playing,
    Helping,
    Lost,
    AskingToQuit,
    Quitting,
}

pub struct G2048GameManager<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    game_state: GameState,
    menu_opt: MenuOpt,
    play_opts: PlayOpt,
    confirmed: bool,
    record: u32,
    board: Board,
    kill_execution: bool,
}

impl<'a> GameManager for G2048GameManager<'a> {
    fn process_events(&mut self) -> Result<()> {
        match self.game_state {
            GameState::Starting => (),
            GameState::Helping => game_manager::read_key()?,
            GameState::Menu | GameState::Lost => self.read_menu_input()?,
            GameState::Playing => self.read_play_input()?,
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
                        self.game_state = GameState::AskingToQuit;
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
                "You Lost!",
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
            confirmed: false,
            record: 0,
            board: Board::new(),
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
        let message = String::from(
            "Picture a tiny grid filled with numbered tiles, and your job is to combine them to 
reach the elusive number 2048.

To play, you'll swipe the tiles in four directions: up, down, left, or right. Each 
time you swipe, all the tiles on the grid move as far as they can in that direction.

Now, here's where the magic happens: when two tiles with the same number collide as
you swipe, they merge into one tile with double the value. For example, if you combine 
two \"2\" tiles, they become a single \"4\" tile.

The goal is to keep combining tiles to create larger and larger numbers until you reach
2048. But beware! With each move you make, a new tile appears on the grid, threatening 
to fill it up and block your path to victory.

The game ends when you can't make any more moves, either because the grid is full or 
because you can't combine any more tiles. So, strategize wisely, plan your moves, and 
see if you can reach that elusive 2048 tile!

It's a simple yet addictive puzzle that'll have you sliding tiles and chasing that 
elusive number for hours on end. Enjoy the challenge!",
        );
        self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(message).white(), area);
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
        loop {
            let event = read()?;
            if should_move_up(&event) {
                self.play_opts = PlayOpt::Direction(Directions::Up);
                break;
            } else if should_move_down(&event) {
                self.play_opts = PlayOpt::Direction(Directions::Down);
                break;
            } else if should_move_left(&event) {
                self.play_opts = PlayOpt::Direction(Directions::Left);
                break;
            } else if should_move_right(&event) {
                self.play_opts = PlayOpt::Direction(Directions::Right);
                break;
            } else if should_quit(&event) {
                self.play_opts = PlayOpt::Quit;
                break;
            } else if should_force_quit(&event) {
                self.kill_execution = true;
                break;
            }
        }
        Ok(())
    }
}
