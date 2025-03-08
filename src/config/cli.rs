use std::{
    convert::Infallible,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{Parser, Subcommand};

use crate::types::bind::Bind;

use super::{full::FullConfig, partial::PartialConfig};

#[derive(Debug, Parser)]
pub struct Cli {
    #[group(flatten)]
    config: PartialConfig,

    /// Set config path
    #[arg(short, long = "config", default_value_t = ConfigDir::default(), value_name = "PATH")]
    config_dir: ConfigDir,
    /// Ignore config files
    #[arg(short = 'C', long)]
    disable_config: bool,

    /// Bind an action to a key
    #[arg(short, long = "bind", value_name = "BIND")]
    binds: Vec<Bind>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Run a script
    Run {
        /// Script name
        script: String,
    },
}

impl Cli {
    pub fn config(self) -> FullConfig {
        FullConfig {
            config: self.config,
            binds: self.binds.into(),
        }
    }

    fn config_dir(&self) -> Option<&Path> {
        self.config_dir
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

    pub fn active_script(&self) -> Option<&str> {
        match &self.command {
            Command::Run { script } => Some(script),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConfigDir(Option<PathBuf>);

impl Default for ConfigDir {
    fn default() -> Self {
        ConfigDir(dirs::config_dir().map(|path| path.join("tez")))
    }
}

impl FromStr for ConfigDir {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ConfigDir(s.parse().ok()))
    }
}

impl Display for ConfigDir {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(path) => path.to_string_lossy().fmt(f),
            None => "not found".fmt(f),
        }
    }
}
