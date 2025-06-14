use std::sync::mpsc;
use std::thread;

use chrono::{Local, TimeZone};
use color_eyre::eyre;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::Widget;

use crate::screens::account::AccountScreen;
use crate::screens::{NavEvent, Screen};
use crate::service::BudgetService;
use crate::types::{AppEvent, Transaction};

#[derive(Debug, PartialEq)]
enum AppState {
    Exited,
    Running,
}

pub struct App {
    state: AppState,
    screen: Box<dyn Screen>,
    frames_count: u32,
    service: BudgetService,
    notifications: Vec<String>,
    events: mpsc::Receiver<AppEvent>,
}

impl App {
    pub fn new() -> eyre::Result<Self> {
        let mut service = BudgetService::new("budget.db")?;
        let (tx, rx) = mpsc::channel();
        let mut s_transactions = AccountScreen::new(tx);
        s_transactions.sync(&mut service)?;

        Ok(Self {
            state: AppState::Running,
            screen: Box::from(s_transactions),
            frames_count: 0,
            service,
            notifications: vec![],
            events: rx,
        })
    }
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> eyre::Result<()> {
        let (tx, rx) = mpsc::channel();

        // read terminal events in separate thread
        thread::spawn(move || {
            while let Ok(event) = event::read() {
                let send_result = tx.send(event);
                if send_result.is_err() {
                    break;
                }
            }
        });

        while self.state != AppState::Exited {
            self.frames_count += 1;
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(&rx)?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.state = AppState::Exited;
    }

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let layout =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.area());

        self.screen.render(layout[0], frame.buffer_mut());

        if let Some(text) = self.notifications.last() {
            Text::from(text.as_str())
                .style(Style::default().fg(Color::Red))
                .render(layout[1], frame.buffer_mut());
        }
    }

    fn handle_events(&mut self, rx: &mpsc::Receiver<Event>) -> Result<(), mpsc::RecvError> {
        let event = rx.recv()?;
        if let Event::Key(key_event) = event {
            if let KeyEventKind::Press = key_event.kind {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(), // default key handling
                    KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                        self.exit()
                    }
                    KeyCode::Char('j') | KeyCode::Down => self.screen.handle_nav(NavEvent::Down),
                    KeyCode::Char('k') | KeyCode::Up => self.screen.handle_nav(NavEvent::Up),
                    KeyCode::Char('l') | KeyCode::Right => self.screen.handle_nav(NavEvent::Rigth),
                    KeyCode::Char('h') | KeyCode::Left => self.screen.handle_nav(NavEvent::Left),
                    KeyCode::Enter => self.screen.handle_nav(NavEvent::Interact),
                    KeyCode::Esc => self.screen.handle_nav(NavEvent::Cancel),
                    KeyCode::Char('g' | 'G') => {
                        self.service
                            .put_trns(&gen_fake_trancations(5))
                            .expect("failed to insert fake data");
                        self.screen
                            .sync(&mut self.service)
                            .expect("Failed to sync data");
                    }
                    _ => {}
                }
            }
        };

        self.screen.handle_event(&event);

        while let Ok(app_event) = self.events.try_recv() {
            let AppEvent::Notifiction(text) = app_event;
            self.notifications.push(text);
        }
        Ok(())
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
