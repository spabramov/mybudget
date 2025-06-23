use std::cell::RefCell;
use std::collections::VecDeque;
use std::{sync::mpsc, thread};

use chrono::{Local, TimeZone};
use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use crate::{
    service::BudgetService,
    types::{AppEvent, NavEvent, Transaction},
    widgets::transactions::TransactionsTableState,
};

mod account;
mod notifications;

#[derive(Debug, PartialEq, Default)]
enum AppState {
    #[default]
    Running,
    Exited,
}

#[derive(Debug, Default, PartialEq)]
enum Screen {
    #[default]
    Account,
}
enum PopUp {
    Notifications,
}

pub struct App {
    state: AppState,

    // main screen and optional pop-up screen
    screen: Screen,
    popup: Option<PopUp>,

    // budget database service
    service: BudgetService,

    // account screen
    transactions: Vec<Transaction>,
    account_state: RefCell<TransactionsTableState>,

    // misc
    frames_count: u32,
    notifications: Vec<String>,
    events: VecDeque<AppEvent>,
}

impl App {
    pub fn new() -> Self {
        let service = BudgetService::new("budget.db");

        Self {
            state: AppState::default(),

            screen: Screen::Account,
            popup: None,

            service,

            transactions: vec![],
            account_state: RefCell::new(TransactionsTableState::default()),
            frames_count: 0,

            notifications: vec![],
            events: VecDeque::new(),
        }
    }
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> eyre::Result<()> {
        let (tx, rx) = mpsc::channel();

        // read terminal events in separate thread
        thread::spawn(move || {
            while let Ok(event) = event::read() {
                if tx.send(event).is_err() {
                    break;
                }
            }
        });

        if let Err(report) = self.screen_sync() {
            self.notify(format!("Error: {report}"));
        }

        while self.state != AppState::Exited {
            self.frames_count += 1;
            terminal.draw(|frame| self.draw(frame))?;
            if let Err(report) = self.handle_events(&rx) {
                self.notify(format!("Error: {report}"))
            }
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.state = AppState::Exited;
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let layout =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.area());

        let content = layout[0];
        let footer = layout[1];

        self.screen_draw(content, frame.buffer_mut());
        self.popup_draw(content, frame.buffer_mut());

        if let Some(text) = self.notifications.last() {
            Line::from(text.as_str())
                .style(Style::default().fg(Color::Red))
                .render(footer, frame.buffer_mut());
        }
    }

    fn handle_events(&mut self, rx: &mpsc::Receiver<Event>) -> eyre::Result<()> {
        if let Event::Key(key_event) = rx.recv()? {
            if let KeyEventKind::Press = key_event.kind {
                // Global key handler

                if self.popup.is_some() {
                    match key_event.code {
                        KeyCode::Char('q' | 'Q') | KeyCode::Esc => self.popup = None,
                        _ => {}
                    }
                } else {
                    match key_event.code {
                        // Global behaviour
                        KeyCode::Char('c' | 'C')
                            if key_event.modifiers == KeyModifiers::CONTROL =>
                        {
                            self.exit()
                        }

                        // Navigation
                        KeyCode::Char('j' | 'J') | KeyCode::Down => self.screen_nav(NavEvent::Down),
                        KeyCode::Char('k' | 'K') | KeyCode::Up => self.screen_nav(NavEvent::Up),
                        KeyCode::Char('l') | KeyCode::Right => self.screen_nav(NavEvent::Rigth),
                        KeyCode::Char('h' | 'H') | KeyCode::Left => self.screen_nav(NavEvent::Left),
                        KeyCode::Enter => self.screen_nav(NavEvent::Interact),
                        KeyCode::Esc => self.screen_nav(NavEvent::Cancel),

                        // Pop-ups
                        KeyCode::Char('n' | 'N') => self.popup = Some(PopUp::Notifications),

                        // Temporary
                        KeyCode::Char('g' | 'G') => {
                            self.notifications
                                .push(String::from("Generating 5 fake transactions"));
                            self.service.put_trns(&gen_fake_trancations(5))?;
                            self.screen_sync()?;
                        }
                        _ => {}
                    }
                    self.screen_input(&key_event);
                }
            }
        };

        while let Some(app_event) = self.events.pop_front() {
            match app_event {
                AppEvent::Notifiction(text) => self.notifications.push(text),
            }
        }
        Ok(())
    }

    fn notify(&mut self, msg: String) {
        self.notifications.push(msg);
    }

    fn screen_sync(&mut self) -> eyre::Result<()> {
        match &self.screen {
            Screen::Account => self.sync_account_screen(),
        }
    }

    fn screen_draw(&self, area: Rect, buf: &mut Buffer) {
        match &self.screen {
            Screen::Account => self.draw_account_screen(area, buf),
        }
    }
    fn popup_draw(&self, area: Rect, buf: &mut Buffer) {
        match &self.popup {
            Some(PopUp::Notifications) => self.draw_notifications_popup(area, buf),
            None => {}
        }
    }

    fn screen_input(&mut self, event: &KeyEvent) {
        match &self.screen {
            Screen::Account => self.input_account_screen(event),
        }
    }

    fn screen_nav(&self, nav_event: NavEvent) {
        match &self.screen {
            Screen::Account => self.nav_account_screen(nav_event),
        }
    }
}

fn gen_fake_trancations(size: u32) -> Vec<Transaction> {
    (0..size)
        .map(|num| {
            let timestamp = Local
                .with_ymd_and_hms(2000 + num as i32, 2, 3, 4, 5, 6)
                .unwrap();

            Transaction {
                transaction_id: None,
                credit_acc_id: Some(1),
                debit_acc_id: Some(2),
                timestamp,
                amount: num as i64 * 100,
                category: Some(String::from(&format!("Category #{}", num + 1))),
                description: Some(String::from(&format!("Desctiption #{}", num + 1))),
            }
        })
        .collect()
}
