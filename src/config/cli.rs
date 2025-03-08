use std::{
    convert::Infallible,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser;

use crate::types::bind::Bind;

use super::{full::FullConfig, partial::PartialConfig};

#[derive(Debug, Parser)]
pub struct Cli {
    #[group(flatten)]
    config: PartialConfig,

    /// Set config file path
    #[arg(short, long = "config", default_value_t = ConfigPath::default())]
    config_path: ConfigPath,
    /// Ignore config file
    #[arg(short = 'C', long)]
    disable_config: bool,

    /// Bind an action to a key
    #[arg(short, long = "bind")]
    binds: Vec<Bind>,
}

impl Cli {
    pub fn config(self) -> FullConfig {
        FullConfig {
            config: self.config,
            binds: self.binds.into(),
        }
    }

    fn config_dir(&self) -> Option<&Path> {
        self.config_path
            .0
            .as_deref()
            .filter(|_| !self.disable_config)
    }

    pub fn config_file(&self) -> Option<PathBuf> {
        self.config_dir().map(|path| path.join("config.toml"))
    }

    pub fn script_dir(&self) -> Option<PathBuf> {
        self.config_dir().map(|path| path.join("scripts"))
    }
}

#[derive(Clone, Debug)]
pub struct ConfigPath(Option<PathBuf>);

impl Default for ConfigPath {
    fn default() -> Self {
        ConfigPath(dirs::config_dir().map(|path| path.join("tez")))
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
