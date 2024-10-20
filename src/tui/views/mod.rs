use std::str::FromStr;

use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, TableState};
use ratatui_view::stateful_table::{IndexedRow, StatefulTable, TableStyle, Tabular};

use crate::globals::CONFIG;

pub mod detailed_item;
pub mod feeds;
pub mod help;
pub mod items;
pub mod links;
pub mod notification;
pub mod prompt;
pub mod quit;
pub mod status;
pub mod tags;

pub fn user_table_style<'a>() -> Option<TableStyle<'a>> {
    let border = match CONFIG.theme.borders {
        true => Some(Block::default().borders(Borders::ALL).border_style(
            Style::default().fg(Color::from_str(&CONFIG.theme.border_color).unwrap()),
        )),
        _ => None,
    };

    let border = match CONFIG.theme.rounded_borders {
        true => border.map(|b| b.border_type(BorderType::Rounded)),
        _ => border,
    };

    Some(TableStyle {
        table: Style::default(),
        header: Style::default()
            .fg(Color::from_str(&CONFIG.theme.fg_header_color).unwrap())
            .bg(Color::from_str(&CONFIG.theme.bg_header_color).unwrap()),
        block: border,
        highlight: Style::default()
            .fg(Color::from_str(&CONFIG.theme.fg_selected_color).unwrap())
            .bg(Color::from_str(&CONFIG.theme.bg_selected_color).unwrap()),
    })
}
pub fn table_style<'a>() -> TableStyle<'a> {
    TableStyle {
        table: Style::default(),
        header: Style::default().yellow(),
        block: Some(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().yellow())
                .border_type(BorderType::Rounded),
        ),
        highlight: Style::default().add_modifier(Modifier::REVERSED).yellow(),
    }
}

pub fn new_table<'a, T: Tabular>(data: Vec<T>, state: TableState) -> StatefulTable<'a, T> {
    StatefulTable::new(data, state, user_table_style().unwrap_or_else(table_style))
}
pub fn new_indexed_table<'a, T: Tabular>(
    data: Vec<T>,
    state: TableState,
) -> StatefulTable<'a, IndexedRow<T>> {
    StatefulTable::new(
        IndexedRow::from(data),
        state,
        user_table_style().unwrap_or_else(table_style),
    )
}
