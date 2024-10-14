use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Row, Table, TableState};
use ratatui::Frame;

use super::{Padding, UiObject};

pub trait Tabular {
    type Value;
    fn value(&self) -> Self::Value;
    fn content(&self) -> Vec<String>;
    fn column_constraints() -> Vec<fn(u16) -> Constraint>;
    fn column_names() -> Option<Vec<String>> {
        None
    }
    fn style(&self) -> Style {
        Style::default()
    }
}
pub trait InteractiveTable {
    fn select_next(&mut self);
    fn select_prev(&mut self);
    fn select_next_page(&mut self);
    fn select_prev_page(&mut self);
    fn select_absolute(&mut self, idx: usize);
    fn select_visible(&mut self, idx: usize);
    fn select_relative(&mut self, offset: isize);
    fn selected_index(&self) -> Option<usize>;
    fn screen_coords_to_row_index(&self, pos: (u16, u16)) -> Option<usize>;
}

pub struct StatefulTable<'a, T: Tabular> {
    table: Table<'a>,
    state: TableState,
    values: Vec<T::Value>,
    area: Option<Rect>,
    padding: Padding,
    inner_width: u16,
}
impl<'a, T: Tabular> StatefulTable<'a, T> {
    pub fn new(
        data: Vec<T>,
        mut state: TableState,
        styler: fn(Table<'a>) -> (Table<'a>, Padding),
    ) -> Self {
        let rows = data
            .iter()
            .map(|data| Row::new(data.content()).style(data.style()));

        let col_widths = Self::get_widths(&data);
        let constraints: Vec<_> = col_widths
            .iter()
            .zip(T::column_constraints().iter())
            .map(|(s, c)| c(*s))
            .collect();

        let table = Table::new(rows, constraints);
        let (table, padding) = styler(table);

        if let Some(idx) = state.selected() {
            state.select(Some(idx.clamp(0, data.len().saturating_sub(1))));
        }

        Self {
            table,
            values: data.iter().map(T::value).collect(),
            state,
            area: None,
            padding,
            inner_width: col_widths.iter().sum::<u16>() + (col_widths.len() - 1) as u16,
        }
    }
    pub fn new_indexed(data: Vec<T>, state: TableState) -> StatefulTable<'a, IndexedRow<T>> {
        StatefulTable::new(IndexedRow::from(data), state, apply_table_style::<T>)
    }
    pub fn selected_value(&self) -> Option<&T::Value> {
        self.state.selected().and_then(|i| self.values.get(i))
    }
    pub fn rows_count(&self) -> usize {
        self.values.len()
    }
    pub fn state(&self) -> TableState {
        self.state.clone()
    }
    pub fn size(&self) -> (u16, u16) {
        (
            self.inner_width + self.padding.l + self.padding.r,
            self.rows_count() as u16 + self.padding.t + self.padding.b,
        )
    }

    fn inner_area(&self) -> Option<Rect> {
        self.area.map(|area| Rect {
            x: area.x + self.padding.l,
            y: area.y + self.padding.t,
            width: area.width - self.padding.l - self.padding.r,
            height: area.height - self.padding.t - self.padding.b,
        })
    }
    fn get_widths(data: &[T]) -> Vec<u16> {
        let mut data: Vec<_> = data.iter().map(T::content).collect();

        if let Some(headers) = T::column_names() {
            data.push(headers);
        }
        if data.is_empty() {
            return vec![];
        }
        let widths = |a: Vec<String>| a.iter().map(|e| e.len() as u16).collect::<Vec<_>>();
        let max_widths = |a: Vec<u16>, b: Vec<u16>| (0..a.len()).map(|i| a[i].max(b[i])).collect();
        data.into_iter().map(widths).reduce(max_widths).unwrap()
    }
}
impl<'a, T: Tabular> InteractiveTable for StatefulTable<'a, T> {
    fn select_next(&mut self) {
        self.select_relative(1);
    }
    fn select_prev(&mut self) {
        self.select_relative(-1);
    }
    fn select_next_page(&mut self) {
        if let Some(area) = self.inner_area() {
            self.select_relative(area.height as isize)
        }
    }
    fn select_prev_page(&mut self) {
        if let Some(area) = self.inner_area() {
            self.select_relative(-(area.height as isize))
        }
    }
    fn select_absolute(&mut self, idx: usize) {
        let idx = idx.clamp(0, self.rows_count().saturating_sub(1));
        self.state.select(Some(idx));
    }
    fn select_visible(&mut self, idx: usize) {
        self.select_absolute(self.state.offset().saturating_add(idx));
    }
    fn select_relative(&mut self, offset: isize) {
        let new = self.selected_index().map_or(0, |curr| {
            if offset < 0 {
                curr.saturating_sub(offset.unsigned_abs())
            } else {
                curr.saturating_add(offset.unsigned_abs())
            }
        });
        self.select_absolute(new);
    }
    fn selected_index(&self) -> Option<usize> {
        self.state.selected()
    }
    fn screen_coords_to_row_index(&self, (row, col): (u16, u16)) -> Option<usize> {
        if let Some(area) = self.inner_area()
            && row >= area.y
            && col >= area.x
            && row < area.y.saturating_add(area.height)
            && col < area.x.saturating_add(area.width)
        {
            let relative = row.saturating_sub(area.y);
            let absolute = relative.saturating_add(self.state.offset() as u16);
            return Some(absolute as usize);
        }
        None
    }
}
impl<'a, T: Tabular> UiObject for StatefulTable<'a, T> {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        self.area = Some(area);
        f.render_stateful_widget(&self.table, area, &mut self.state);
    }
    fn handle_event(&mut self, ev: &Event) {
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::PageDown => self.select_next_page(),
                KeyCode::PageUp => self.select_prev_page(),
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.select_prev(),
                _ => {}
            },
            Event::Mouse(ev) => {
                let pos = (ev.row, ev.column);
                match ev.kind {
                    MouseEventKind::ScrollDown => match ev.modifiers {
                        KeyModifiers::ALT => self.select_relative(2),
                        _ => self.select_next(),
                    },
                    MouseEventKind::ScrollUp => match ev.modifiers {
                        KeyModifiers::ALT => self.select_relative(-2),
                        _ => self.select_prev(),
                    },
                    MouseEventKind::Down(MouseButton::Left | MouseButton::Right) => {
                        if let Some(row) = self.screen_coords_to_row_index(pos) {
                            self.select_absolute(row);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub struct IndexedRow<T: Tabular> {
    pub idx: usize,
    pub data: T,
}
impl<T: Tabular> IndexedRow<T> {
    pub fn from(data: Vec<T>) -> Vec<IndexedRow<T>> {
        data.into_iter()
            .enumerate()
            .map(|(i, d)| IndexedRow { idx: i, data: d })
            .collect()
    }
}
impl<T: Tabular> Tabular for IndexedRow<T> {
    type Value = T::Value;
    fn value(&self) -> Self::Value {
        self.data.value()
    }
    fn content(&self) -> Vec<String> {
        let mut content = self.data.content();
        content.insert(0, format!("{}", self.idx));
        content
    }
    fn column_names() -> Option<Vec<String>> {
        T::column_names().map(|mut names| {
            names.insert(0, format!("#"));
            names
        })
    }
    fn column_constraints() -> Vec<fn(u16) -> Constraint> {
        let mut constraints = T::column_constraints();
        constraints.insert(0, Constraint::Length);
        constraints
    }
    fn style(&self) -> Style {
        T::style(&self.data)
    }
}

// --- hack

pub fn apply_table_style<T: Tabular>(mut table: Table<'_>) -> (Table<'_>, Padding) {
    let mut pad = Padding::default();

    let block = {
        pad.add(1);
        Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().yellow())
    };

    table = table
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED).yellow());
    if let Some(header) = T::column_names() {
        pad.t += 1;
        table = table.header(Row::new(header).yellow());
    }
    (table, pad)
}
