use crossterm::event::Event;
use ratatui::{
    style::palette::tailwind,
    widgets::{Block, Borders, ScrollbarState, StatefulWidget, TableState},
};
use tui_input::backend::crossterm::EventHandler;

use super::utils;
use crate::types::{ScreenMode, Transaction};
use ratatui::{
    prelude::*,
    widgets::{HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, Table},
};

const TABLE_TITLE: &str = "Transactions";
const TABLE_TITLE_BOTTOM: &str = " ← ↑ ↓ → to move selection ";
const TABLE_HEADER: [&str; 4] = ["Date", "Category", "Description", "Amount"];

const ROW_HEIGHT: u16 = 1;
const ROW_HIGHLIGHT_SYMBOL: &str = " > ";

const COLUMN_SPACING: u16 = 1;
const COLUMN_WIDTHS: [Constraint; 4] = [
    Constraint::Length(12),
    Constraint::Fill(1),
    Constraint::Fill(9),
    Constraint::Min(13),
];
const COLUMN_ALIGNMENTS: [Alignment; 4] = [
    Alignment::Left,
    Alignment::Left,
    Alignment::Left,
    Alignment::Right,
];

#[derive(Debug, Default)]
pub struct TransactionsTableState {
    pub mode: ScreenMode,
    table_state: TableState,
    scroll_state: ScrollbarState,
    input: tui_input::Input,
    size: usize,
}

#[derive(Debug)]
pub struct TransactionsTable<'a> {
    items: &'a [Transaction],
}

impl StatefulWidget for TransactionsTable<'_> {
    type State = TransactionsTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TransactionsTableState) {
        self.render_table(area, buf, state);
        self.render_scrollbar(area, buf, state);
    }
}

impl<'a> TransactionsTable<'a> {
    pub fn new(items: &'a [Transaction]) -> Self {
        Self { items }
    }

    fn render_table(&self, area: Rect, buf: &mut Buffer, state: &mut TransactionsTableState) {
        let header_style = Style::default().add_modifier(Modifier::REVERSED);
        let selected_cell_style = match state.mode {
            ScreenMode::Browsing => Style::default().bg(tailwind::GRAY.c600),
            ScreenMode::Editing => Style::default().bg(tailwind::GRAY.c600).fg(Color::Yellow),
        };

        let widths: Vec<_> = Layout::horizontal(COLUMN_WIDTHS)
            .spacing(COLUMN_SPACING)
            .horizontal_margin(1 + ROW_HIGHLIGHT_SYMBOL.len() as u16)
            .split(area)
            .iter()
            .map(|part| part.width)
            .collect();

        let header = TABLE_HEADER
            .into_iter()
            .map(Text::from)
            .zip(COLUMN_ALIGNMENTS)
            .map(|(text, align)| text.alignment(align))
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows = self.items.iter().enumerate().map(|(row, data)| {
            let color = match row % 2 {
                0 => Color::default(),
                _ => tailwind::GRAY.c800,
            };

            to_text_iter(data)
                .zip(&widths)
                .enumerate()
                .map(|(col, (text, &width))| {
                    if state.mode == ScreenMode::Editing
                        && Some(col) == state.table_state.selected_column()
                        && Some(row) == state.table_state.selected()
                    {
                        utils::to_text_with_cursor(&state.input, width)
                    } else {
                        text
                    }
                })
                .zip(COLUMN_ALIGNMENTS)
                .map(|(text, align)| text.alignment(align))
                .collect::<Row>()
                .style(Style::default().bg(color))
                .height(ROW_HEIGHT)
        });

        let table = Table::new(rows, COLUMN_WIDTHS)
            .header(header)
            .cell_highlight_style(selected_cell_style)
            .highlight_symbol(ROW_HIGHLIGHT_SYMBOL)
            .highlight_spacing(HighlightSpacing::Always)
            .column_spacing(COLUMN_SPACING)
            .block(
                Block::default()
                    .title(TABLE_TITLE)
                    .title_bottom(Line::from(TABLE_TITLE_BOTTOM).right_aligned())
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            );

        StatefulWidget::render(table, area, buf, &mut state.table_state);
    }

    fn render_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &mut TransactionsTableState) {
        if area.height as usize <= state.size * ROW_HEIGHT as usize {
            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .thumb_symbol("▐")
                .track_symbol(None)
                .begin_symbol(None)
                .end_symbol(None);
            let area = area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            });

            StatefulWidget::render(scrollbar, area, buf, &mut state.scroll_state);
        }
    }
}

impl TransactionsTableState {
    pub fn new(size: usize) -> Self {
        Self {
            mode: ScreenMode::Browsing,
            table_state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(size),
            size,
            input: tui_input::Input::default(),
        }
    }

    pub fn start_editing(&mut self) {
        if let (Some(_), Some(_)) = self.selected() {
            self.mode = ScreenMode::Editing
        }
    }

    pub fn accept_edit(&mut self) {
        if let (Some(_row), Some(_col)) = self.selected() {
            let _value = self.input.value_and_reset();
        }
        self.mode = ScreenMode::Browsing;
    }

    pub fn cancel_edit(&mut self) {
        self.input.reset();
        self.mode = ScreenMode::Browsing;
    }

    pub fn select(&mut self, row: Option<usize>, column: Option<usize>) {
        self.table_state.select(row);
        self.table_state.select_column(column);
        self.scroll_state = self.scroll_state.position(row.unwrap_or(0));
    }
    pub fn selected(&self) -> (Option<usize>, Option<usize>) {
        (
            self.table_state.selected(),
            self.table_state.selected_column(),
        )
    }

    pub fn deselect(&mut self) {
        self.table_state.select_column(None);
    }

    pub fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) if i < self.size => i + 1,
            _ => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    pub fn previous_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    pub fn next_column(&mut self) {
        self.table_state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.table_state.select_previous_column();
    }

    pub fn handle_input(&mut self, event: &Event) {
        self.input.handle_event(event);
    }
}

fn to_text_iter(data: &Transaction) -> impl Iterator<Item = Text> {
    let category: &str = data.category.as_ref().map_or("", |x| x);
    let description: &str = data.description.as_ref().map_or("", |x| x);
    let amount_whole = data.amount / 100;
    let amount_frac = data.amount % 100;

    [
        Text::from(format!("{}", data.timestamp.format("%Y-%m-%d"))),
        Text::from(category),
        Text::from(description),
        Text::from(format!("{amount_whole}.{amount_frac:02}")),
    ]
    .into_iter()
}
