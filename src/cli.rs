use anyhow::Result;
use clap::Parser;

use crate::{config::Config, utils::Extent};

#[derive(Parser)]
pub struct Cli {
    /// Set viewport height
    #[arg(short = 'H', long, value_parser = clap::value_parser!(Extent))]
    height: Option<Extent>,
}

impl Cli {
    pub fn config(&self) -> Result<Config> {
        Ok(Config {
            height: self.height,
        })
    }
}
