use std::{fs, ops::Deref, path::Path};

use anyhow::Result;
use serde::Deserialize;

use crate::types::{action::Action, key::Key};

use super::{binds::Binds, partial::PartialConfig};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FullConfig {
    #[serde(flatten)]
    pub config: PartialConfig,
    #[serde(default = "Binds::new")]
    pub binds: Binds,
}

impl FullConfig {
    pub fn parse<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn action(&self, key: &Key) -> Option<&Action> {
        self.binds.get(key)
    }
}

impl Deref for FullConfig {
    type Target = PartialConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}
