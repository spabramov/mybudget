use app::App;
use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::io;
use std::{sync::mpsc, thread};
use types::AppEvent;

mod app;
mod screens;
mod types;
mod widgets;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || -> io::Result<()> { handle_input(tx) });

    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal, rx);

    ratatui::restore();
    result

    // not waiting on event_thread, just dropping and stopping it
}

fn handle_input(tx: mpsc::Sender<AppEvent>) -> io::Result<()> {
    loop {
        let app_event: Option<AppEvent> = match event::read()? {
            Event::Resize(_, _) => Some(AppEvent::Resize),
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                        Some(AppEvent::Quit)
                    }
                    KeyCode::Char(ch) => Some(AppEvent::KeyEvent(ch)),
                    KeyCode::Enter => Some(AppEvent::Accept),
                    KeyCode::Esc => Some(AppEvent::Cancel),
                    _ => None,
                }
            }
            _ => None,
        };

        if let Some(app_event) = app_event {
            if tx.send(app_event).is_err() {
                break;
            }
        }
    }
    Ok(())
}
