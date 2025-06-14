use std::sync::mpsc;
use std::thread;

use chrono::{Local, TimeZone};
use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::Widget;

use crate::screens::account::AccountScreen;
use crate::screens::notifications::NotificationsScreen;
use crate::screens::{NavEvent, Screen};
use crate::service::BudgetService;
use crate::types::{AppEvent, ScreenEnum, Transaction};

#[derive(Debug, PartialEq, Default)]
enum AppState {
    #[default]
    Running,
    Exited,
}

pub struct App {
    state: AppState,
    screens: Vec<(ScreenEnum, Box<dyn Screen>)>,
    frames_count: u32,
    notifications: Vec<String>,
    events: (mpsc::Sender<AppEvent>, mpsc::Receiver<AppEvent>),
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            state: AppState::default(),
            screens: vec![],
            frames_count: 0,
            notifications: vec![],
            events: (tx, rx),
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

        self.load_screen(ScreenEnum::default())?;

        while self.state != AppState::Exited {
            self.frames_count += 1;
            terminal.draw(|frame| self.draw(frame))?;
            if let Err(report) = self.handle_events(&rx) {
                self.notify(format!("{report}"))
            }
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.state = AppState::Exited;
    }

    fn load_screen(&mut self, kind: ScreenEnum) -> eyre::Result<()> {
        if Some(&kind) == self.screens.last().map(|(kind, _)| kind) {
            // the same screen type
            return Ok(());
        }
        let mut screen: Box<dyn Screen> = match kind {
            ScreenEnum::Account => {
                let service = BudgetService::new("budget.db")?;
                Box::from(AccountScreen::new(self.events.0.clone(), service))
            }
            ScreenEnum::Notifications => Box::from(NotificationsScreen::new(
                self.events.0.clone(),
                self.notifications.clone(),
            )),
        };
        screen.sync()?;
        self.screens.push((kind, screen));
        Ok(())
    }

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let layout =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.area());

        for (_, screen) in &mut self.screens {
            screen.render(layout[0], frame.buffer_mut());
        }

        if let Some(text) = self.notifications.last() {
            Line::from(text.as_str())
                .style(Style::default().fg(Color::Red))
                .render(layout[1], frame.buffer_mut());
        }
    }

    fn handle_events(&mut self, rx: &mpsc::Receiver<Event>) -> eyre::Result<()> {
        let event = rx.recv()?;

        if let Some((_, ref mut screen)) = self.screens.last_mut() {
            screen.handle_event(&event)?;

            if let Event::Key(key_event) = event {
                if let KeyEventKind::Press = key_event.kind {
                    // Global key handler
                    match key_event.code {
                        KeyCode::Char('c' | 'C')
                            if key_event.modifiers == KeyModifiers::CONTROL =>
                        {
                            self.exit()
                        }
                        KeyCode::Char('j' | 'J') | KeyCode::Down => {
                            screen.handle_nav(NavEvent::Down)?
                        }
                        KeyCode::Char('k' | 'K') | KeyCode::Up => {
                            screen.handle_nav(NavEvent::Up)?
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            screen.handle_nav(NavEvent::Rigth)?
                        }
                        KeyCode::Char('h' | 'H') | KeyCode::Left => {
                            screen.handle_nav(NavEvent::Left)?
                        }
                        KeyCode::Enter => screen.handle_nav(NavEvent::Interact)?,
                        KeyCode::Esc => screen.handle_nav(NavEvent::Cancel)?,
                        KeyCode::Char('n' | 'N') => self.load_screen(ScreenEnum::Notifications)?,

                        KeyCode::Char('g' | 'G') => {
                            self.notifications
                                .push(String::from("Generating 5 fake transactions"));
                            BudgetService::new("budget.db")?.put_trns(&gen_fake_trancations(5))?;
                            screen.sync()?;
                        }
                        _ => {}
                    }
                }
            }
        };

        while let Ok(app_event) = self.events.1.try_recv() {
            match app_event {
                AppEvent::Notifiction(text) => self.notifications.push(text),
                AppEvent::ExitScreen => {
                    if !self.screens.is_empty() {
                        self.screens.pop();
                    }
                    if self.screens.is_empty() {
                        self.state = AppState::Exited;
                    }
                }
            }
        }
        Ok(())
    }

    fn notify(&mut self, msg: String) {
        self.notifications.push(msg);
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
