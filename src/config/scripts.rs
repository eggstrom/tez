use std::{collections::HashMap, fs, ops::Deref, path::Path, rc::Rc};

use anyhow::{anyhow, Result};
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
        let mut scripts = HashMap::new();
        if let Ok(configs) = path.as_ref().read_dir() {
            for config in configs.filter_map(Result::ok) {
                let config = ScriptConfig::parse(config.path())?;
                scripts.insert(Rc::clone(&config.name), Rc::new(config));
            }
        }
        Ok(Scripts(scripts))
    }

    pub fn get(&self, name: &str) -> Result<Rc<ScriptConfig>> {
        self.0
            .get(name)
            .map(Rc::clone)
            .ok_or_else(|| anyhow!("couldn't find script `{name}`"))
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
