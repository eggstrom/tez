use app::App;
use ratatui::style::Stylize;

mod actions;
mod app;
mod tui;

fn main() {
    if let Err(error) = App::default().run() {
        eprintln!("{}: {error}", "error:".red());
    };
}
