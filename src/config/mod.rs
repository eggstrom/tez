use std::{fs, ops::Deref, path::Path};

use anyhow::Result;
use binds::Binds;
use clap::Parser;
use cli::Cli;
use common::CommonConfig;
use serde::Deserialize;

use crate::types::{action::Action, key::Key};

mod binds;
mod cli;
mod common;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    common: CommonConfig,
    #[serde(default)]
    binds: Binds,
}

impl Config {
    pub fn load() -> Result<Self> {
        let cli = Cli::parse();
        let mut config = cli
            .config_path()
            .map(Config::parse)
            .unwrap_or(Ok(Config::default()))?;
        config = config.overwrite(&cli.config());
        config.insert_default_binds();
        Ok(config)
    }

    fn parse<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }

    fn overwrite(&self, other: &Self) -> Self {
        let mut new = self.clone();
        new.common.overwrite(&other.common);
        new.binds.overwrite(&other.binds);
        new
    }

    pub fn insert_default_binds(&mut self) {
        if !self.common.disable_default_binds {
            self.binds.insert_defaults();
        }
    }

    pub fn action_for_key(&self, key: &Key) -> Option<Action> {
        self.binds.action_for_key(key)
    }
}

impl Deref for Config {
    type Target = CommonConfig;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}
