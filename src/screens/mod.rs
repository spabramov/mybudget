use crate::types::AppEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub mod account;

pub trait Screen {
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn handle_event(&mut self, event: &AppEvent) -> bool;
}
