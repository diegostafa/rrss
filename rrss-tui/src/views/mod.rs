use ratatui::layout::Rect;

pub mod detailed_item;
pub mod feeds;
pub mod help;
pub mod items;
pub mod links;
pub mod popup;
pub mod prompt;
pub mod quit;
pub mod tags;

pub fn centered_rect(area: Rect, (width, height): (u16, u16)) -> Rect {
    Rect {
        x: (area.x + area.width / 2).saturating_sub(width / 2),
        y: (area.y + area.height / 2).saturating_sub(height / 2),
        width,
        height,
    }
}
