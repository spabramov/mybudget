use std::{rc::Rc, usize::MAX};

use ratatui::{
    style::palette::tailwind,
    widgets::{ScrollbarState, StatefulWidget, TableState},
};

use crate::types::Transaction;
use ratatui::{
    prelude::*,
    widgets::{Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, Table},
};

const ITEM_HEIGHT: u16 = 3;

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
        self.render_table(area, buf, state);
        self.render_scrollbar(area, buf, state);
    }
}

impl TransactionsTable {
    fn render_table(&self, area: Rect, buf: &mut Buffer, state: &mut TransactionsTableState) {
        let header_style = Style::default().add_modifier(Modifier::REVERSED);
        let selected_cell_style = Style::default()
            .bg(tailwind::GRAY.c600)
            // .add_modifier(Modifier::REVERSED)
        ;

        let header = ["Date", "Amount", "Description"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = state.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => Color::default(),
                _ => tailwind::GRAY.c800,
            };

            Row::new(vec![
                Cell::from(Text::from(format!("\n {} \n", data.timestamp))),
                Cell::from(Text::from(format!("\n {:.2} \n", data.amount)).right_aligned()),
                Cell::from(Text::from(format!("\n {} \n", data.description))),
            ])
            .style(Style::default().bg(color))
            .height(ITEM_HEIGHT)
        });

        let bar = " > ";
        let tb = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(21),
                Constraint::Min(5),
                Constraint::Percentage(80),
            ],
        )
        .header(header)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec!["".into(), bar.into()]))
        .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(tb, area, buf, &mut state.table_state);
    }

    fn render_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &mut TransactionsTableState) {
        StatefulWidget::render(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
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
            scroll_state: ScrollbarState::default(),
            items: data.clone(),
        }
    }

    pub fn deselect(&mut self) {
        self.table_state.select_column(None);
    }

    pub fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT as usize);
    }

    pub fn previous_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT as usize);
    }

    pub fn next_column(&mut self) {
        self.table_state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.table_state.select_previous_column();
    }
}
