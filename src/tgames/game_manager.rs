use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::Result;

pub trait GameManager {
    fn run(&mut self) -> Result<bool> {
        while !self.ended() {
            self.process_events()?;
            self.update()?;
            self.render()?;
            self.limit_fps();
        }
        Ok(self.kill_execution())
    }
    fn process_events(&mut self) -> Result<()>;
    fn update(&mut self) -> Result<()>;
    fn render(&mut self) -> Result<()>;
    fn ended(&self) -> bool;
    fn kill_execution(&self) -> bool;
    // In case is needed to limit game fps.
    fn limit_fps(&self) {}
}

// Useful things
pub fn read_key() -> Result<()> {
    event::read()?;
    Ok(())
}

pub fn read_confirmation(key: &Event) -> bool {
    !matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Char('N'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn confirmation_guide() -> String {
    String::from("N or n - go back to playing\nAny key - confirm")
}

#[derive(Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub enum Difficult {
    Easy,
    Medium,
    Hard,
}

pub fn should_move_up(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn should_move_down(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn should_move_right(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn should_move_left(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn should_force_quit(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn should_quit(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn should_play(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Char('p'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn should_help(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('?'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn should_increase_fps(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('F'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::SHIFT,
            ..
        })
    )
}

pub fn should_decrease_fps(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('f'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        })
    )
}

pub fn should_pause(key: &Event) -> bool {
    matches!(
        key,
        Event::Key(KeyEvent {
            code: KeyCode::Char('p'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        })
    )
}
