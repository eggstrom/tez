use anyhow::Result;
use clap::Parser;

use crate::{config::Config, utils::Extent};

#[derive(Parser)]
pub struct Cli {
    /// Set viewport X position
    #[arg(short)]
    x: Option<Extent>,
    /// Set viewport Y position
    #[arg(short)]
    y: Option<Extent>,
    /// Set viewport width
    #[arg(short = 'W', long)]
    width: Option<Extent>,
    /// Set viewport height
    #[arg(short = 'H', long)]
    height: Option<Extent>,
}

impl Cli {
    pub fn config(&self) -> Result<Config> {
        Ok(Config {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        })
    }
}
