use std::{ops::Deref, rc::Rc};

use anyhow::Result;
use binds::Binds;
use clap::Parser;
use cli::Cli;
use full::FullConfig;
use partial::PartialConfig;
use scripts::{ScriptConfig, Scripts};

use crate::types::{action::Action, key::Key};

mod binds;
mod cli;
mod full;
mod partial;
mod scripts;

#[derive(Clone, Debug, Default)]
pub struct Config {
    default_binds: Binds,
    main_config: FullConfig,
    script_configs: Scripts,
    active_script_config: Option<Rc<ScriptConfig>>,
    cli_config: FullConfig,
    active_config: PartialConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let cli = Cli::parse();
        let main_config = cli
            .config_file()
            .map(FullConfig::parse)
            .unwrap_or(Ok(FullConfig::default()))?;
        let script_configs = cli
            .script_dir()
            .map(Scripts::load)
            .unwrap_or(Ok(Scripts::default()))?;
        let active_script_config = cli
            .active_script()
            .map(|name| script_configs.get(name))
            .transpose()?;
        let cli_config = cli.config();
        let active_config = match &active_script_config {
            Some(script_config) => main_config.overwrite(script_config).overwrite(&cli_config),
            None => main_config.overwrite(&cli_config),
        };

        Ok(Config {
            default_binds: Binds::default(),
            main_config,
            cli_config,
            script_configs,
            active_script_config,
            active_config,
        })
    }

    pub fn action(&self, key: &Key) -> Option<&Action> {
        self.cli_config
            .action(key)
            .or_else(|| {
                self.active_script_config
                    .as_ref()
                    .and_then(|script| script.action(key))
            })
            .or_else(|| self.main_config.action(key))
            .or_else(|| {
                (!self.active_config.disable_default_binds)
                    .then(|| self.default_binds.get(key))
                    .flatten()
            })
    }
}

impl Deref for Config {
    type Target = PartialConfig;

    fn deref(&self) -> &Self::Target {
        &self.active_config
    }
}
