use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::StatefulWidget;

use crate::service::BudgetService;
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
        let table_state = TransactionsTableState::new(0);

        Self {
            table_state,
            items: vec![],
        }
    }

    pub fn sync(&mut self, service: &mut BudgetService) {
        self.items = match service.get_trns() {
            Ok(trns) => trns,
            Err(err) => panic!("Failed to sync transactions: {err:?}"),
        };
        let selected = self.table_state.selected();
        self.table_state = TransactionsTableState::new(self.items.len());
        self.table_state.select(selected);
    }
}

impl Screen for AccountScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        TransactionsTable::new(&self.items).render(area, buf, &mut self.table_state);
    }

    fn handle_event(&mut self, event: &AppEvent) -> bool {
        match event {
            AppEvent::Key('j') | AppEvent::Down => self.table_state.next_row(),
            AppEvent::Key('k') | AppEvent::Up => self.table_state.previous_row(),
            AppEvent::Key('l') | AppEvent::Rigth => self.table_state.next_column(),
            AppEvent::Key('h') | AppEvent::Left => self.table_state.previous_column(),
            AppEvent::Cancel => self.table_state.deselect(),
            _ => return false,
        }
        true
    }
}
