#![feature(let_chains)]
#![warn(unused_results)]

use app::App;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::Rect;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use rrss_core::feed_manager::FeedManager;

pub mod app;
pub mod keymaps;
pub mod theme;
pub mod views;
pub mod widgets;

fn main() {
    App::new(FeedManager::new()).init().run().unwrap()
}

pub fn try_init_term() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, Box<std::io::Error>> {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}
pub fn try_release_term(
    mut term: Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<std::io::Error>> {
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
