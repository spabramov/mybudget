use ratatui::{
    style::palette::tailwind,
    widgets::{Block, Borders, ScrollbarState, StatefulWidget, TableState},
};

use crate::types::Transaction;
use ratatui::{
    prelude::*,
    widgets::{Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, Table},
};

const TABLE_TITLE: &str = "Transactions";
const TABLE_HEADER: [&str; 4] = [" Date", " Category", " Description", "Amount "];

const ROW_HEIGHT: u16 = 3;
const ROW_HIGHLIGHT_SYMBOL: &str = "\n > ";

const INSTRUCTIONS_TEXT: &str = " ← ↑ ↓ → to move selection ";

#[derive(Debug, Default)]
pub struct TransactionsTableState {
    table_state: TableState,
    scroll_state: ScrollbarState,
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
        let selected_cell_style = Style::default().bg(tailwind::GRAY.c600);

        let header = get_header_row(TABLE_HEADER).style(header_style).height(1);

        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => Color::default(),
                _ => tailwind::GRAY.c800,
            };

            get_item_row(data)
                .style(Style::default().bg(color))
                .height(ROW_HEIGHT)
        });

        let tb = Table::new(
            rows,
            [
                Constraint::Length(12),
                Constraint::Fill(1),
                Constraint::Fill(9),
                Constraint::Min(13),
            ],
        )
        .header(header)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(ROW_HIGHLIGHT_SYMBOL)
        .highlight_spacing(HighlightSpacing::Always)
        .block(
            Block::default()
                .title(TABLE_TITLE)
                .title_bottom(Line::from(INSTRUCTIONS_TEXT).right_aligned())
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        );

        StatefulWidget::render(tb, area, buf, &mut state.table_state);
    }

    fn render_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &mut TransactionsTableState) {
        if area.height as usize <= state.size * ROW_HEIGHT as usize {
            StatefulWidget::render(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .thumb_symbol("▐")
                    .track_symbol(None)
                    .begin_symbol(None)
                    .end_symbol(None),
                area.inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                buf,
                &mut state.scroll_state,
            );
        }
    }
}

impl TransactionsTableState {
    pub fn new(size: usize) -> Self {
        Self {
            table_state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(size),
            size,
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.table_state.select(index);
        self.scroll_state = self.scroll_state.position(index.unwrap_or(0));
    }
    pub fn selected(&self) -> Option<usize> {
        self.table_state.selected()
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
}

fn get_header_row(header: [&str; 4]) -> Row {
    Row::new(vec![
        Cell::from(Text::from(header[0])),
        Cell::from(Text::from(header[1])),
        Cell::from(Text::from(header[2])),
        Cell::from(Text::from(header[3]).right_aligned()),
    ])
}

fn get_item_row(data: &Transaction) -> Row {
    let category: &str = data.category.as_ref().map_or("", |x| x);
    let description: &str = data.description.as_ref().map_or("", |x| x);
    let amount_whole = data.amount / 100;
    let amount_frac = data.amount % 100;

    Row::new(vec![
        Cell::from(Text::from(format!(
            "\n {} \n",
            data.timestamp.format("%Y-%m-%d")
        ))),
        Cell::from(Text::from(format!("\n {} \n", category))),
        Cell::from(Text::from(format!("\n {} \n", description))),
        Cell::from(Text::from(format!("\n {amount_whole}.{amount_frac:02} \n")).right_aligned()),
    ])
}
