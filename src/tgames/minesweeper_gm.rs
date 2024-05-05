mod board;

use super::game_manager;
use board::Board;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::Stdout;

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
    Won,
    Lost,
    Quitting,
}

pub struct MinesweeperGameManager<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    game_state: GameState,
    menu_opt: MenuOpt,
    board: Board,
}

impl<'a> game_manager::GameManager for MinesweeperGameManager<'a> {
    fn process_events(&mut self) -> std::io::Result<()> {
        todo!()
    }
    fn update(&mut self) -> std::io::Result<()> {
        todo!()
    }
    fn render(&mut self) -> std::io::Result<()> {
        todo!()
    }
    fn ended(&self) -> bool {
        todo!()
    }
}

impl<'a> MinesweeperGameManager<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        todo!()
    }
}
