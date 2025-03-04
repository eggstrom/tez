use anyhow::Result;
use app::App;
use clap::Parser;
use cli::Cli;
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
    if let Err(error) = run().await {
        eprintln!("{} {error}", "error:".red());
    };
}

async fn run() -> Result<()> {
    let config = Cli::parse().config()?;
    App::new(config)?.run().await?;
    Ok(())
}
