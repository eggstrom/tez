use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crossterm::event::{KeyCode as K, KeyModifiers as M};
use derive_more::From;
use serde::Deserialize;

use crate::types::{
    action::{Action, InputAction, TuiAction},
    bind::Bind,
    key::Key,
};

#[derive(Clone, Debug, Deserialize, From, PartialEq)]
pub struct Binds(HashMap<Key, Action>);

impl Binds {
    pub fn new() -> Self {
        Binds(HashMap::new())
    }
}

impl Default for Binds {
    #[rustfmt::skip]
    fn default() -> Self {
        let map = HashMap::from([
            (Key::new(K::Char('c'), M::CONTROL), Action::Exit),
            (Key::new(K::Char('n'), M::CONTROL), TuiAction::Next.into()),
            (Key::new(K::Char('p'), M::CONTROL), TuiAction::Previous.into()),
            (Key::new(K::Char('a'), M::ALT), TuiAction::First.into()),
            (Key::new(K::Char('e'), M::ALT), TuiAction::Last.into()),

            (Key::new(K::Left, M::NONE), InputAction::MoveBack.into()),
            (Key::new(K::Down, M::NONE), InputAction::MoveDown.into()),
            (Key::new(K::Up, M::NONE), InputAction::MoveUp.into()),
            (Key::new(K::Right, M::NONE), InputAction::MoveForward.into()),
            (Key::new(K::Left, M::CONTROL), InputAction::MoveBackWord.into()),
            (Key::new(K::Down, M::CONTROL), InputAction::MoveToBottom.into()),
            (Key::new(K::Up, M::CONTROL), InputAction::MoveToTop.into()),
            (Key::new(K::Right, M::CONTROL), InputAction::MoveToEndOfWord.into()),
            (Key::new(K::Home, M::NONE), InputAction::MoveToHead.into()),
            (Key::new(K::End, M::NONE), InputAction::MoveToEnd.into()),
            (Key::new(K::Char('f'), M::CONTROL), InputAction::MoveForward.into()),
            (Key::new(K::Char('b'), M::CONTROL), InputAction::MoveBack.into()),
            (Key::new(K::Char('f'), M::ALT), InputAction::MoveToEndOfWord.into()),
            (Key::new(K::Char('b'), M::ALT), InputAction::MoveBackWord.into()),
            (Key::new(K::Char('a'), M::CONTROL), InputAction::MoveToHead.into()),
            (Key::new(K::Char('e'), M::CONTROL), InputAction::MoveToEnd.into()),

            (Key::new(K::Backspace, M::NONE), InputAction::Delete.into()),
            (Key::new(K::Backspace, M::CONTROL), InputAction::DeleteWord.into()),
            (Key::new(K::Delete, M::NONE), InputAction::DeleteNext.into()),
            (Key::new(K::Delete, M::CONTROL), InputAction::DeleteNextWord.into()),
            (Key::new(K::Char('d'), M::CONTROL), InputAction::DeleteNext.into()),
            (Key::new(K::Char('d'), M::ALT), InputAction::DeleteNextWord.into()),
            (Key::new(K::Char('w'), M::CONTROL), InputAction::DeleteWord.into()),
            (Key::new(K::Char('u'), M::CONTROL), InputAction::DeleteToHead.into()),
            (Key::new(K::Char('k'), M::CONTROL), InputAction::DeleteToEnd.into()),
        ]);
        Binds(map)
    }
}

impl Deref for Binds {
    type Target = HashMap<Key, Action>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Binds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Bind>> for Binds {
    fn from(value: Vec<Bind>) -> Self {
        let map = value
            .into_iter()
            .map(|bind| (bind.key, bind.action))
            .collect();
        Binds(map)
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
}
