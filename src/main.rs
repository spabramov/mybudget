use app::App;
use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::{sync::mpsc, thread};
use types::AppEvent;

mod app;
mod screens;
mod service;
mod types;
mod widgets;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || -> eyre::Result<()> { handle_input(tx) });

    let mut terminal = ratatui::init();
    let result = App::new()?.run(&mut terminal, rx);

    ratatui::restore();
    result
}

fn handle_input(tx: mpsc::Sender<AppEvent>) -> eyre::Result<()> {
    loop {
        let event = event::read()?;
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('c' | 'C'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            // Ctrl + C
            tx.send(AppEvent::Quit)?;
        };

        tx.send(AppEvent::TermEvent(event))?;
    }
}
