use crate::{
    app::App,
    types::{AppEvent, NavEvent, ScreenMode},
    widgets::transactions::{TransactionsTable, TransactionsTableState},
};
use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Clear, StatefulWidget, Widget},
};

impl App {
    pub(super) fn draw_account_screen(&self, area: Rect, buf: &mut Buffer) {
        let mut state = self.account_state.borrow_mut();

        let table = TransactionsTable::new(&self.transactions);
        Clear.render(area, buf);
        StatefulWidget::render(table, area, buf, &mut state);
    }

    pub(super) fn sync_account_screen(&mut self) -> eyre::Result<()> {
        let selected = self.account_state.borrow().selected();

        self.transactions = self.service.get_trns()?;
        let mut new_state = TransactionsTableState::new(self.transactions.len());
        new_state.select(selected.0, selected.1);

        self.account_state.replace(new_state);

        Ok(())
    }

    pub(super) fn input_account_screen(&mut self, key_event: &KeyEvent) {
        self.account_state.borrow_mut().handle_input(key_event);
        if self.account_state.borrow().mode == ScreenMode::Browsing {
            match key_event.code {
                KeyCode::Char('q' | 'Q') => self.exit(),
                KeyCode::Char('d' | 'D') => self.delete_selected_trns(),
                _ => {}
            };
        }
    }
    pub(super) fn nav_account_screen(&self, event: NavEvent) {
        let _value = self.account_state.borrow_mut().navigate(event);
    }

    fn delete_selected_trns(&mut self) {
        if let (Some(row), _) = self.account_state.borrow().selected() {
            let trn = &self.transactions[row];
            if let Some(trn_id) = trn.transaction_id {
                if let Err(report) = self.service.del_trns(&[trn_id]) {
                    self.events
                        .push_back(AppEvent::Notifiction(format!("Error: {report}")))
                }
            }
            self.events.push_back(AppEvent::Notifiction(format!(
                "Deleting transaction {:?}",
                &trn.transaction_id
            )));
            self.transactions.remove(row);
        }
    }
}
