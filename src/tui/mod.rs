use std::io;

use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::Rect;
use ratatui::prelude::CrosstermBackend;
use ratatui::{crossterm, Terminal};

pub mod app;
mod keymaps;
mod theme;
mod views;
mod widgets;

pub fn try_init_term() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<io::Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}
pub fn try_release_term(
    mut term: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<io::Error>> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;
    Ok(())
}

pub fn centered_rect(area: Rect, (width, height): (u16, u16)) -> Rect {
    Rect {
        x: (area.x + area.width / 2).saturating_sub(width / 2),
        y: (area.y + area.height / 2).saturating_sub(height / 2),
        width,
        height,
    }
}
