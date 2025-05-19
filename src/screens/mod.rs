use crate::service::BudgetService;

pub mod account;

pub trait Screen {
    fn render(&mut self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer);
    fn handle_event(&mut self, event: &crossterm::event::Event) -> bool;
    fn sync(&mut self, service: &mut BudgetService) -> color_eyre::eyre::Result<()>;
}
