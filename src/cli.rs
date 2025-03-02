use anyhow::Result;
use clap::Parser;

use crate::{
    config::Config,
    types::{alignment::Alignment, extent::Extent},
};

#[derive(Parser)]
pub struct Cli {
    /// Set viewport width
    #[arg(short = 'W', long)]
    width: Option<Extent>,
    /// Set viewport height
    #[arg(short = 'H', long)]
    height: Option<Extent>,
    /// Set viewport alignment
    #[arg(short = 'A', long, default_value_t = Alignment::default())]
    alignment: Alignment,
}

impl Cli {
    pub fn config(&self) -> Result<Config> {
        Ok(Config {
            width: self.width,
            height: self.height,
            alignment: self.alignment,
        })
    }
}
