use std::cell::RefCell;
use std::sync::mpsc;

use color_eyre::eyre;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Clear, StatefulWidget, Widget};

use crate::service::BudgetService;
use crate::types::{AppEvent, ScreenMode, Transaction};
use crate::widgets::transactions::{TransactionsTable, TransactionsTableState};

use super::{NavEvent, Screen};

pub struct AccountScreen {
    service: BudgetService,
    items: Vec<Transaction>,
    table_state: RefCell<TransactionsTableState>,
    events: mpsc::Sender<AppEvent>,
}

impl AccountScreen {
    pub fn new(tx: mpsc::Sender<AppEvent>, service: BudgetService) -> Self {
        Self {
            service,
            items: Vec::default(),
            table_state: RefCell::new(TransactionsTableState::default()),
            events: tx,
        }
    }
}

impl Screen for AccountScreen {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let mut state = self.table_state.borrow_mut();
        TransactionsTable::new(&self.items).render(area, buf, &mut state);
    }

    fn handle_nav(&self, event: NavEvent) -> eyre::Result<()> {
        let mut state = self.table_state.borrow_mut();
        match state.mode {
            ScreenMode::Browsing => match event {
                NavEvent::Left => state.previous_column(),
                NavEvent::Rigth => state.next_column(),
                NavEvent::Up => state.previous_row(),
                NavEvent::Down => state.next_row(),
                NavEvent::Cancel => state.deselect(),
                NavEvent::Interact => state.start_editing(),
            },
            ScreenMode::Editing => match event {
                NavEvent::Cancel => state.cancel_edit(),
                NavEvent::Interact => state.accept_edit(),
                _ => {
                    // suppress navigation in Edit Mode
                }
            },
        }
        Ok(())
    }

    fn handle_event(&mut self, term_event: &Event) -> eyre::Result<()> {
        if matches!(self.table_state.borrow().mode, ScreenMode::Editing) {
            self.table_state.borrow_mut().handle_input(term_event);
        } else if let Event::Key(key_event) = term_event {
            if let KeyEventKind::Press = key_event.kind {
                match key_event.code {
                    KeyCode::Char('q') => self.events.send(AppEvent::ExitScreen)?, // default key handling
                    KeyCode::Char('d' | 'D') => self.delete_selected_trns(),

                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn sync(&mut self) -> eyre::Result<()> {
        self.items = self.service.get_trns()?;

        let selected = self.table_state.borrow().selected();
        self.table_state = RefCell::new(TransactionsTableState::new(self.items.len()));
        self.table_state.borrow_mut().select(selected.0, selected.1);
        Ok(())
    }
}

impl AccountScreen {
    pub fn delete_selected_trns(&mut self) {
        if let (Some(row), _) = self.table_state.borrow().selected() {
            let trn = &self.items[row];
            if let Some(trn_id) = trn.transaction_id {
                if let Err(report) = self.service.del_trns(&[trn_id]) {
                    self.events
                        .send(AppEvent::Notifiction(format!("Error: {report}")))
                        .expect("Failed to send AppEvent back to App")
                }
            }
            self.items.remove(row);
            self.events
                .send(AppEvent::Notifiction(format!(
                    "Deleting transaction {row:?}"
                )))
                .expect("Failed to send AppEvent back to App")
        }
    }
}
