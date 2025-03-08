use std::{
    convert::Infallible,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser;

use crate::{config::Config, types::bind::Bind};

use super::common::CommonConfig;

#[derive(Debug, Parser)]
pub struct Cli {
    #[group(flatten)]
    common: CommonConfig,
    /// Set config file path
    #[arg(short, long, default_value_t = ConfigPath::default())]
    config: ConfigPath,
    /// Ignore config file
    #[arg(short = 'C', long)]
    disable_config: bool,

    /// Bind an action to a key
    #[arg(short, long = "bind")]
    binds: Vec<Bind>,
}

impl Cli {
    pub fn config(self) -> Config {
        Config {
            common: self.common,
            binds: self.binds.into(),
        }
    }

    pub fn config_path(&self) -> Option<&Path> {
        self.config.0.as_deref().filter(|_| !self.disable_config)
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
