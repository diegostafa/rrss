use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, TableState};
use ratatui_helpers::stateful_table::{IndexedRow, Padding, StatefulTable, TableStyle, Tabular};
use rrss_core::globals::CONFIG;

pub struct StyledWidget;
impl StyledWidget {
    pub fn header_paragraph<'a>(s: String) -> Paragraph<'a> {
        Paragraph::new(s)
            .fg(CONFIG.theme.fg_item_header)
            .bg(CONFIG.theme.bg_item_header)
    }
    pub fn table<'a, T: Tabular>(
        data: Vec<T>,
        state: TableState,
        title: Option<String>,
    ) -> StatefulTable<'a, T> {
        StatefulTable::new(data, state, Self::table_style(), title)
    }
    pub fn indexed_table<'a, T: Tabular>(
        data: Vec<T>,
        state: TableState,
        title: Option<String>,
    ) -> StatefulTable<'a, IndexedRow<T>> {
        StatefulTable::new(IndexedRow::from(data), state, Self::table_style(), title)
    }
    pub fn block<'a>() -> Block<'a> {
        let mut block = Block::new();
        if CONFIG.theme.borders {
            block = block
                .borders(Borders::ALL)
                .border_style(Style::default().fg(CONFIG.theme.border_color))
        }
        if CONFIG.theme.rounded_borders {
            block = block.border_type(BorderType::Rounded)
        }
        block
    }
    pub fn table_padding<'a>() -> Padding {
        let mut padding = Padding::default();
        if CONFIG.theme.borders {
            padding.add_value(1);
        }
        padding
    }
    fn table_style<'a>() -> TableStyle<'a> {
        TableStyle {
            table: Style::default(),
            header: Style::default()
                .fg(CONFIG.theme.fg_header_color)
                .bg(CONFIG.theme.bg_header_color),
            block: (Self::block(), Self::table_padding()),
            highlight: Style::default()
                .fg(CONFIG.theme.fg_selected_color)
                .bg(CONFIG.theme.bg_selected_color),
            normal: Style::default()
                .fg(CONFIG.theme.fg_normal_color)
                .bg(CONFIG.theme.bg_normal_color),
            column_spacing: CONFIG.theme.column_spacing,
            col_highlight: Style::default(),
        }
    }
}
