use std::{
    collections::HashMap,
    fs,
    ops::{Deref, DerefMut},
    path::Path,
    rc::Rc,
};

use anyhow::Result;
use derive_more::From;
use serde::{Deserialize, Deserializer};

use super::full::FullConfig;

#[derive(Clone, Debug, Default, From)]
pub struct Scripts(HashMap<Rc<str>, Rc<ScriptConfig>>);

impl Scripts {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut scripts = Scripts::default();
        if let Ok(configs) = path.as_ref().read_dir() {
            for config in configs.filter_map(Result::ok) {
                let config = ScriptConfig::parse(config.path())?;
                scripts.insert(Rc::clone(&config.name), Rc::new(config));
            }
        }
        Ok(scripts)
    }
}

impl Deref for Scripts {
    type Target = HashMap<Rc<str>, Rc<ScriptConfig>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scripts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de> Deserialize<'de> for Scripts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = HashMap::<_, _>::deserialize(deserializer)?
            .into_iter()
            .map(|(name, script)| (name, Rc::new(script)))
            .collect();
        Ok(Scripts(map))
    }
}

#[derive(Debug, Deserialize)]
pub struct ScriptConfig {
    #[serde(flatten)]
    config: FullConfig,
    name: Rc<str>,
    #[serde(default)]
    adjacent_scripts: Vec<String>,
}

impl ScriptConfig {
    pub fn parse<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }
}

impl Deref for ScriptConfig {
    type Target = FullConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}
