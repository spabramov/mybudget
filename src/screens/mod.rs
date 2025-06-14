use crate::service::BudgetService;

pub mod account;

pub enum NavEvent {
    Left,
    Rigth,
    Up,
    Down,
    Cancel,
    Interact,
}
pub trait Screen {
    fn render(&mut self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer);
    fn handle_nav(&mut self, event: NavEvent);
    fn handle_event(&mut self, event: &crossterm::event::Event);
    fn sync(&mut self, service: &mut BudgetService) -> color_eyre::eyre::Result<()>;
}
