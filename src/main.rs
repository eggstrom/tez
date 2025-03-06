use app::App;
use crossterm::style::Stylize;

mod app;
mod cli;
mod config;
mod events;
mod searcher;
mod state;
mod tui;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    if let Err(error) = App::run().await {
        eprintln!("{} {error}", "error:".red());
    };
}
