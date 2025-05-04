use std::sync::mpsc;

use color_eyre::eyre;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::widgets::{Block, Clear, Widget};

use crate::screens::account::AccountScreen;
use crate::screens::Screen;
use crate::types::AppEvent;

type Input = mpsc::Receiver<AppEvent>;

#[derive(Debug, PartialEq)]
enum AppState {
    Exited,
    Quitting,
    Running,
}

enum AppScreen {
    Accounts,
}

pub struct App {
    state: AppState,
    screen: AppScreen,
    s_transactions: AccountScreen,
    has_changes: bool,
    frames_count: u32,
}

impl App {
    pub fn new() -> App {
        Self {
            state: AppState::Running,
            screen: AppScreen::Accounts,
            s_transactions: AccountScreen::new(),
            has_changes: false,
            frames_count: 0,
        }
    }
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal, rx: Input) -> eyre::Result<()> {
        while self.state != AppState::Exited {
            self.frames_count += 1;
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            self.handle_events(&rx)?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.state = if self.has_changes {
            AppState::Quitting
        } else {
            AppState::Exited
        }
    }

    fn handle_events(&mut self, rx: &Input) -> Result<(), mpsc::RecvError> {
        match rx.recv()? {
            AppEvent::Quit => self.exit(),
            AppEvent::Resize => { /* redraw */ }
            app_event => self.handle_event(app_event),
        }
        Ok(())
    }

    fn handle_event(&mut self, app_event: AppEvent) {
        if self.state == AppState::Quitting {
            match app_event {
                AppEvent::Key('y' | 'Y') | AppEvent::Accept => self.state = AppState::Exited,
                AppEvent::Key('n' | 'N') | AppEvent::Cancel => self.state = AppState::Running,
                _ => {}
            }
        } else {
            let consumed = match self.screen {
                AppScreen::Accounts => self.s_transactions.handle_event(&app_event),
            };

            if !consumed {
                if let AppEvent::Key('q') = app_event {
                    // default key handling
                    self.exit()
                }
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self.screen {
            AppScreen::Accounts => self.s_transactions.render(area, buf),
        }

        if self.state == AppState::Quitting {
            let block = Block::bordered().title("Quit?").title_bottom("y / n");
            let area = popup_area(area, 60, 20);

            Clear.render(area, buf);
            block.render(area, buf);
        }
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
