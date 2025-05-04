use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::StatefulWidget;

use crate::types::{AppEvent, Transaction};
use crate::widgets::transactions::{TransactionsTable, TransactionsTableState};

use super::Screen;

#[derive(Debug)]
pub struct AccountScreen {
    table_state: TransactionsTableState,
    items: Vec<Transaction>,
}

impl AccountScreen {
    pub fn new() -> Self {
        let items = gen_fake_trancations();
        let table_state = TransactionsTableState::new(items.len());

        Self { table_state, items }
    }
}

impl Screen for AccountScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        TransactionsTable::new(&self.items).render(area, buf, &mut self.table_state);
    }

    fn handle_event(&mut self, event: &AppEvent) -> bool {
        match event {
            AppEvent::KeyEvent('j') | AppEvent::Down => self.table_state.next_row(),
            AppEvent::KeyEvent('k') | AppEvent::Up => self.table_state.previous_row(),
            AppEvent::KeyEvent('l') | AppEvent::Rigth => self.table_state.next_column(),
            AppEvent::KeyEvent('h') | AppEvent::Left => self.table_state.previous_column(),
            AppEvent::Cancel => self.table_state.deselect(),
            _ => return false,
        }
        true
    }
}

fn gen_fake_trancations() -> Vec<Transaction> {
    (0..22)
        .into_iter()
        .map(|num| {
            Transaction::new(
                &format!("2{num:03}-01-01 00:01:00"),
                (num as f32) * 100.0,
                &format!("Desctiption #{}", num + 1),
            )
        })
        .collect()
}
