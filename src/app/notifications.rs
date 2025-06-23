use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, List, Padding, Widget},
};

use super::App;

impl App {
    pub(super) fn draw_notifications_popup(&self, area: Rect, buf: &mut Buffer) {
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
        Widget::render(list, area, buf);
    }
}
