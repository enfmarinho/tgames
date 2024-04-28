mod g2048_gm;
mod game_manager;
mod snake_gm;
mod tetris_gm;

use self::{
    g2048_gm::G2048GameManager, game_manager::GameManager, snake_gm::SnakeGameManager,
    tetris_gm::TetrisGameManager,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{Result, Stdout};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter, FromRepr};

#[derive(EnumIter, FromRepr, Display, EnumCountMacro, PartialEq)]
pub enum Games {
    Snake,
    Tetris,
    #[strum(to_string = "2048")]
    G2048,
    #[strum(to_string = "")]
    None,
}

pub fn run(terminal: Terminal<CrosstermBackend<Stdout>>, game: Games) -> Result<()> {
    let mut game_instance = TGamesManager::new(terminal);
    if Games::None != game {
        game_instance.run_game(game)?;
    }
    while !game_instance.ended() {
        game_instance.process_events()?;
        game_instance.update()?;
        game_instance.render()?;
    }
    Ok(())
}

enum MainMenuOpts {
    Play,
    Quit,
    Up,
    Down,
    None,
}

enum TGamesState {
    Starting,
    MainMenu,
    Quitting,
}

struct TGamesManager {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    execution_state: TGamesState,
    main_menu_opts: MainMenuOpts,
    game_index: usize,
}

impl TGamesManager {
    fn new(terminal: Terminal<CrosstermBackend<Stdout>>) -> TGamesManager {
        TGamesManager {
            terminal,
            execution_state: TGamesState::Starting,
            main_menu_opts: MainMenuOpts::None,
            game_index: 0,
        }
    }

    fn process_events(&mut self) -> Result<()> {
        match self.execution_state {
            TGamesState::MainMenu => self.read_main_menu_input()?,
            TGamesState::Starting | TGamesState::Quitting => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        match self.execution_state {
            TGamesState::Starting => self.execution_state = TGamesState::MainMenu,
            TGamesState::MainMenu => match self.main_menu_opts {
                MainMenuOpts::Play => self.play()?,
                MainMenuOpts::Quit => self.execution_state = TGamesState::Quitting,
                MainMenuOpts::Up => {
                    if self.game_index > 0 {
                        self.game_index -= 1;
                    }
                }
                MainMenuOpts::Down => {
                    if self.game_index < Games::COUNT - 2 {
                        self.game_index += 1;
                    }
                }
                MainMenuOpts::None => (),
            },
            TGamesState::Quitting => (),
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        match self.execution_state {
            TGamesState::MainMenu => self.display_main_menu()?,
            TGamesState::Starting | TGamesState::Quitting => (),
        }
        Ok(())
    }

    fn ended(&self) -> bool {
        matches!(self.execution_state, TGamesState::Quitting)
    }

    fn display_main_menu(&mut self) -> Result<()> {
        let mut lines: Vec<Line> = Vec::new();
        for (index, opts) in Games::iter().enumerate() {
            if index == self.game_index {
                lines.push(Line::from(Span::styled(
                    "> ".to_owned() + &opts.to_string() + " <",
                    Style::default().green(),
                )));
            } else {
                lines.push(Line::from(
                    Span::styled(opts.to_string(), Style::default()).gray(),
                ));
            }
            lines.push(Line::from(Span::styled("\n\n", Style::default()).gray()));
        }
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
                Paragraph::new(lines).block(
                    Block::new()
                        .borders(Borders::ALL)
                        // .border_type(ratatui::widgets::BorderType::Rounded)
                        .title("tgames")
                        .title_alignment(Alignment::Center),
                ),
                layout[0],
            );

            frame.render_widget(
                Paragraph::new(Self::tips_message()).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Tips")
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[0],
            );

            frame.render_widget(
                Paragraph::new(Self::keybingins_guide()).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Keybindings")
                        .title_alignment(Alignment::Center),
                ),
                sub_layout[1],
            );
        })?;
        Ok(())
    }

    fn keybingins_guide() -> String {
        String::from(
            "ENTER       - Play\nw or k or  - Go up\ns or j or  - Go down\nESC or q    - Quit\n",
        )
    }

    fn tips_message() -> String {
        String::from("TODO")
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
                    self.main_menu_opts = MainMenuOpts::Quit;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    self.main_menu_opts = MainMenuOpts::Play;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                })
                | Event::Key(KeyEvent {
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
                    self.main_menu_opts = MainMenuOpts::Up;
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                    ..
                })
                | Event::Key(KeyEvent {
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
                    self.main_menu_opts = MainMenuOpts::Down;
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn play(&mut self) -> Result<()> {
        match Games::from_repr(self.game_index) {
            Some(game) => self.run_game(game)?,
            None => self.game_index = 0,
        }
        Ok(())
    }

    fn run_game(&mut self, game: Games) -> Result<()> {
        match game {
            Games::Snake => SnakeGameManager::new(&mut self.terminal).run()?,
            Games::Tetris => TetrisGameManager::new(&mut self.terminal).run()?,
            Games::G2048 => G2048GameManager::new(&mut self.terminal).run()?,
            Games::None => (),
        }
        Ok(())
    }
}
