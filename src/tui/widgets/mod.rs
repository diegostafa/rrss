use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use ratatui::layout::Rect;
use ratatui::Frame;
use stateful_table::{InteractiveTable, StatefulTable, Tabular};

pub mod multiline_paragraph;

pub trait UiObject {
    fn draw(&mut self, f: &mut Frame, area: Rect);
    fn handle_event(&mut self, ev: &Event);
}

pub fn handle_table_events<'a, T: Tabular>(table: &mut StatefulTable<'a, T>, ev: &Event) {
    match ev {
        Event::Key(ev) => match ev.code {
            KeyCode::PageDown => table.select_next_page(),
            KeyCode::PageUp => table.select_prev_page(),
            KeyCode::Char('j') | KeyCode::Down => table.select_next(),
            KeyCode::Char('k') | KeyCode::Up => table.select_prev(),
            _ => {}
        },
        Event::Mouse(ev) => {
            let pos = (ev.row, ev.column);
            match ev.kind {
                MouseEventKind::ScrollDown => match ev.modifiers {
                    KeyModifiers::ALT => table.select_relative(2),
                    _ => table.select_next(),
                },
                MouseEventKind::ScrollUp => match ev.modifiers {
                    KeyModifiers::ALT => table.select_relative(-2),
                    _ => table.select_prev(),
                },
                MouseEventKind::Down(MouseButton::Left | MouseButton::Right) => {
                    if let Some(row) = table.screen_coords_to_row_index(pos) {
                        table.select_absolute(row);
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}
