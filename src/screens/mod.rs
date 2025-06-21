use color_eyre::eyre;

pub mod account;
pub mod notifications;

pub enum NavEvent {
    Left,
    Rigth,
    Up,
    Down,
    Cancel,
    Interact,
}
pub trait Screen {
    fn render(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer);
    fn handle_nav(&self, event: NavEvent) -> eyre::Result<()>;
    fn handle_event(&mut self, _event: &crossterm::event::Event) -> eyre::Result<()> {
        Ok(())
    }

    fn sync(&mut self) -> color_eyre::eyre::Result<()> {
        Ok(())
    }
}
