use std::rc::Rc;

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
const TABLE_HEADER: [&str; 3] = [" Date", " Description", "Amount "];

const ROW_HEIGHT: u16 = 3;
const ROW_HIGHLIGHT_SYMBOL: &str = "\n >";

const INSTRUCTIONS_TEXT: &str = " ↑ ↓ to select row | ← → to select column ";

#[derive(Debug, Default)]
pub struct TransactionsTableState {
    pub table_state: TableState,
    pub scroll_state: ScrollbarState,
    items: Rc<Vec<Transaction>>,
}

#[derive(Debug)]
pub struct TransactionsTable;

impl StatefulWidget for TransactionsTable {
    type State = TransactionsTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TransactionsTableState) {
        let layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Fill(1), Constraint::Length(1)],
        )
        .split(area);

        self.render_table(area, buf, state);
        self.render_scrollbar(area, buf, state);
    }
}

impl TransactionsTable {
    fn render_table(&self, area: Rect, buf: &mut Buffer, state: &mut TransactionsTableState) {
        let header_style = Style::default().add_modifier(Modifier::REVERSED);
        let selected_cell_style = Style::default().bg(tailwind::GRAY.c600);

        let header = get_header(TABLE_HEADER).style(header_style).height(1);

        let rows = state.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => Color::default(),
                _ => tailwind::GRAY.c800,
            };

            get_row(data)
                .style(Style::default().bg(color))
                .height(ROW_HEIGHT)
        });

        let tb = Table::new(
            rows,
            [
                Constraint::Length(21),
                Constraint::Percentage(80),
                Constraint::Min(5),
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
        StatefulWidget::render(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
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

impl TransactionsTableState {
    pub fn new(data: &Rc<Vec<Transaction>>) -> Self {
        Self {
            table_state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((data.len() - 1) * ROW_HEIGHT as usize),
            items: data.clone(),
        }
    }

    pub fn deselect(&mut self) {
        self.table_state.select_column(None);
    }

    pub fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) if i < self.items.len() => i + 1,
            _ => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ROW_HEIGHT as usize);
    }

    pub fn previous_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ROW_HEIGHT as usize);
    }

    pub fn next_column(&mut self) {
        self.table_state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.table_state.select_previous_column();
    }
}

fn get_header(header: [&str; 3]) -> Row {
    Row::new(vec![
        Cell::from(Text::from(header[0])),
        Cell::from(Text::from(header[1])),
        Cell::from(Text::from(header[2]).right_aligned()),
    ])
}

fn get_row(data: &Transaction) -> Row {
    Row::new(vec![
        Cell::from(Text::from(format!("\n {} \n", data.timestamp))),
        Cell::from(Text::from(format!("\n {} \n", data.description))),
        Cell::from(Text::from(format!("\n {:.2} \n", data.amount)).right_aligned()),
    ])
}
