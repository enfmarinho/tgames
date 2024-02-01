mod lgames;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use lgames::game_manager::GameManager;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut lgames = lgames::LGamesManager::new(terminal);
    lgames.run()?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
