use std::sync::mpsc;

use color_eyre::eyre;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::StatefulWidget;

use crate::service::BudgetService;
use crate::types::{AppEvent, ScreenMode, Transaction};
use crate::widgets::transactions::{TransactionsTable, TransactionsTableState};

use super::{NavEvent, Screen};

#[derive(Debug)]
pub struct AccountScreen {
    items: Vec<Transaction>,
    table_state: TransactionsTableState,
    events: mpsc::Sender<AppEvent>,
}

impl AccountScreen {
    pub fn new(tx: mpsc::Sender<AppEvent>) -> Self {
        Self {
            items: Vec::default(),
            table_state: TransactionsTableState::default(),
            events: tx,
        }
    }
}

impl Screen for AccountScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        TransactionsTable::new(&self.items).render(area, buf, &mut self.table_state);
    }

    fn handle_nav(&mut self, event: NavEvent) {
        match self.table_state.mode {
            ScreenMode::Browsing => match event {
                NavEvent::Left => self.table_state.previous_column(),
                NavEvent::Rigth => self.table_state.next_column(),
                NavEvent::Up => self.table_state.previous_row(),
                NavEvent::Down => self.table_state.next_row(),
                NavEvent::Cancel => self.table_state.deselect(),
                NavEvent::Interact => self.table_state.start_editing(),
            },
            ScreenMode::Editing => match event {
                NavEvent::Cancel => self.table_state.cancel_edit(),
                NavEvent::Interact => self.table_state.accept_edit(),
                _ => {
                    // suppress navigation in Edit Mode
                }
            },
        }
    }

    fn handle_event(&mut self, term_event: &Event) {
        if let ScreenMode::Editing = self.table_state.mode {
            self.table_state.handle_input(term_event);
        } else if let Event::Key(key_event) = term_event {
            if let KeyEventKind::Press = key_event.kind {
                match key_event.code {
                    KeyCode::Char('d' | 'D') => self.delete_selected_trns(),
                    _ => {}
                }
            }
        }
    }

    fn sync(&mut self, service: &mut BudgetService) -> eyre::Result<()> {
        self.items = service.get_trns()?;

        let selected = self.table_state.selected();
        self.table_state = TransactionsTableState::new(self.items.len());
        self.table_state.select(selected.0, selected.1);
        Ok(())
    }
}

impl AccountScreen {
    pub fn delete_selected_trns(&mut self) {
        if let (Some(row), _) = self.table_state.selected() {
            self.items.remove(row);
            self.events
                .send(AppEvent::Notifiction(format!(
                    "Deleting transaction {row:?}"
                )))
                .expect("Failed to send AppEvent back to App")
        }
    }
}
