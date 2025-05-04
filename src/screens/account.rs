use std::fmt::format;
use std::rc::Rc;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::StatefulWidget;

use crate::types::{AppEvent, Transaction};
use crate::widgets::transactions::{TransactionsTable, TransactionsTableState};

use super::Screen;

#[derive(Debug)]
pub struct AccountScreen {
    table_state: TransactionsTableState,
}

impl AccountScreen {
    pub fn new() -> Self {
        let table_state = TransactionsTableState::new(&Rc::new(gen_fake_trancations()));

        Self { table_state }
    }
}

impl AccountScreen {}

impl Screen for AccountScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        TransactionsTable.render(area, buf, &mut self.table_state);
    }

    fn handle_event(&mut self, event: &AppEvent) -> bool {
        match event {
            AppEvent::KeyEvent('j') => self.table_state.next_row(),
            AppEvent::KeyEvent('k') => self.table_state.previous_row(),
            AppEvent::KeyEvent('l') => self.table_state.next_column(),
            AppEvent::KeyEvent('h') => self.table_state.previous_column(),
            AppEvent::Cancel => self.table_state.deselect(),
            _ => return false,
        }
        true
    }
}

fn gen_fake_trancations() -> Vec<Transaction> {
    (0..30)
        .into_iter()
        .map(|num| {
            Transaction::new(
                &format!("2{num:03}-01-01 00:01:00"),
                (num as f32) * 100f32,
                &format!("Desctiption #{}", num + 1),
            )
        })
        .collect()
}
