use std::collections::HashMap;

use crossterm::event::{KeyCode as K, KeyModifiers as M};
use derive_more::From;
use serde::Deserialize;

use crate::types::{
    action::{Action, TuiAction},
    key::Key,
};

#[derive(Clone, Debug, Default, Deserialize, From, PartialEq)]
pub struct Binds(HashMap<Key, Action>);

impl Binds {
    pub fn action_for_key(&self, key: &Key) -> Option<Action> {
        self.0.get(key).cloned()
    }
}

impl Binds {
    /// Sets default binds if the key isn't already taken by another bind.
    #[rustfmt::skip]
    pub fn insert_defaults(&mut self) {
        for (key, action) in [
            (Key::new(K::Char('c'), M::CONTROL), Action::Exit),
            (Key::new(K::Char('n'), M::CONTROL), TuiAction::Next.into()),
            (Key::new(K::Char('p'), M::CONTROL), TuiAction::Previous.into()),
            (Key::new(K::Char('a'), M::ALT), TuiAction::First.into()),
            (Key::new(K::Char('e'), M::ALT), TuiAction::Last.into()),
        ] {
            self.0.entry(key).or_insert(action);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let parsed = toml::from_str::<Binds>("'ctrl+c' = 'exit'");
        assert_eq!(
            parsed,
            Ok(HashMap::from([(Key::new(K::Char('c'), M::CONTROL), Action::Exit)]).into())
        );
    }

    #[test]
    fn insert_defaults() {
        let mut binds = toml::from_str::<Binds>("'ctrl+c' = 'next'").unwrap();
        let mut empty_binds = Binds::default();
        binds.insert_defaults();
        empty_binds.insert_defaults();

        assert_eq!(
            binds.action_for_key(&Key::new(K::Char('c'), M::CONTROL)),
            Some(TuiAction::Next.into())
        );
        assert_eq!(
            empty_binds.action_for_key(&Key::new(K::Char('c'), M::CONTROL)),
            Some(Action::Exit)
        );
    }
}
