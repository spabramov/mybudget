use std::sync::mpsc;

use color_eyre::eyre;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, Padding, Widget};
use ratatui::{layout::Margin, text::Line};

use crate::types::AppEvent;

use super::Screen;

pub struct NotificationsScreen {
    notifications: Vec<String>,
    events: mpsc::Sender<AppEvent>,
}

impl NotificationsScreen {
    pub fn new(events: mpsc::Sender<AppEvent>, notifications: Vec<String>) -> Self {
        Self {
            events,
            notifications,
        }
    }
}

impl Screen for NotificationsScreen {
    fn render(&mut self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let area = area.inner(Margin::new(20, 3));
        let block = Block::default()
            .padding(Padding::new(1, 1, 0, 0))
            .title("Notification")
            .title_bottom(Line::from(" <Esc> to close this window ").right_aligned())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let list = List::from_iter(
            self.notifications
                .iter()
                .rev()
                .take(area.height as usize)
                .map(|line| Line::from(line.as_str().fg(Color::Red))),
        )
        .block(block);

        Clear.render(area, buf);
        list.render(area, buf);
    }

    fn handle_nav(&mut self, event: super::NavEvent) -> eyre::Result<()> {
        if let super::NavEvent::Cancel = event {
            self.events.send(AppEvent::ExitScreen)?
        }
        Ok(())
    }

    fn handle_event(&mut self, event: &crossterm::event::Event) -> eyre::Result<()> {
        if let Event::Key(key_event) = event {
            if let KeyEventKind::Press = key_event.kind {
                match key_event.code {
                    KeyCode::Char('q') => self.events.send(AppEvent::ExitScreen)?, // default key handling
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
