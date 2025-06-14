use app::App;
use color_eyre::eyre;

mod app;
mod screens;
mod service;
mod types;
mod widgets;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let result = App::new()?.run(&mut terminal);

    ratatui::restore();
    result
}
