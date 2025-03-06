use std::{
    convert::Infallible,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser;

use crate::{
    config::Config,
    types::{alignment::Alignment, extent::Extent},
};

#[derive(Debug, Parser)]
pub struct Cli {
    /// Set config file path
    #[arg(short, long, default_value_t = ConfigPath::default())]
    config: ConfigPath,

    /// Set viewport width
    #[arg(short = 'W', long)]
    width: Option<Extent>,
    /// Set viewport height
    #[arg(short = 'H', long)]
    height: Option<Extent>,
    /// Set viewport alignment
    #[arg(short = 'A', long)]
    alignment: Option<Alignment>,
}

impl Cli {
    pub fn config(&self) -> Config {
        let mut config = Config::default();
        config.set_width(self.width);
        config.set_height(self.height);
        config.set_alignment(self.alignment);
        config
    }

    pub fn config_path(&self) -> Option<&Path> {
        self.config.0.as_deref()
    }
}

#[derive(Clone, Debug)]
pub struct ConfigPath(Option<PathBuf>);

impl Default for ConfigPath {
    fn default() -> Self {
        ConfigPath(dirs::config_dir().map(|path| path.join("tez").join("config.toml")))
    }
}

impl FromStr for ConfigPath {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ConfigPath(s.parse().ok()))
    }
}

impl Display for ConfigPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(path) => path.to_string_lossy().fmt(f),
            None => "not found".fmt(f),
        }
    }
}
