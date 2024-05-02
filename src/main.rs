mod tgames;
use crossterm::{
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{
    env::{self},
    io::{self, stdout, Result},
};
use tgames::Games;

fn main() -> Result<()> {
    let mut game = Games::None;
    let argv: Vec<String> = env::args().collect();
    if argv.len() > 1 {
        if argv[1] == "snake" {
            game = Games::Snake;
        } else if argv[1] == "tetris" {
            game = Games::Tetris;
        } else if argv[1] == "2048" {
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

    tgames::run(terminal, game)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    print_goodbye_message();
    Ok(())
}

fn print_help_message() {
    println!("Invalid command-line argument!");
    println!("Press enter to continue or CRTL-c to end execution.");
}

fn print_goodbye_message() {
    println!(
        "{}",
        "Thanks for playing tgames󰊴, hope you enjoyed it!".green()
    );
}
