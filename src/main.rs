use anyhow::Result;
use app::App;
use clap::Parser;
use cli::Cli;
use crossterm::style::Stylize;

mod actions;
mod app;
mod cli;
mod config;
mod searcher;
mod state;
mod tui;
mod utils;

fn main() {
    if let Err(error) = run() {
        eprintln!("{} {error}", "error:".red());
    };
}

fn run() -> Result<()> {
    let config = Cli::parse().config()?;
    App::new(config)?.run()?;
    Ok(())
}
