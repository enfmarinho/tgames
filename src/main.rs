mod lgames;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use lgames::Games;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{
    env::{self},
    io::{self, stdout, Result},
};

fn main() -> Result<()> {
    let mut game = Games::None;
    let argv: Vec<String> = env::args().collect();
    if argv.len() > 1 {
        if argv[1] == "--snake" {
            game = Games::Snake;
        } else if argv[1] == "--tetris" {
            game = Games::Tetris;
        } else if argv[1] == "--2048" {
            game = Games::G2048;
        } else {
            print_help_message();
            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }
    }

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    lgames::run(terminal, game)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn print_help_message() {
    println!("Invalid command-line argument!");
    println!("Press enter to continue or CRTL-c to end execution.");
}
