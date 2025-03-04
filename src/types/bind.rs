use std::str::FromStr;

use derive_more::From;
use thiserror::Error;

use crate::utils::StrExt;

use super::{
    action::{Action, ParseActionError},
    key::{Key, ParseKeyError},
};

#[derive(Debug, PartialEq)]
pub struct Bind {
    key: Key,
    action: Action,
}

impl Bind {
    pub fn new(key: Key, action: Action) -> Self {
        Bind { key, action }
    }

    fn parse(s: &str) -> Result<Self, ParseBindError> {
        let mut colon = s
            .find_last_adjacent(':')
            .ok_or(ParseBindError::InvalidFormat)?;
        colon = s.char_indices().nth(colon).unwrap().0;
        let (key, action) = s.split_at(colon);
        Ok(Bind {
            key: key.parse()?,
            action: action.get(1..).unwrap_or("").parse()?,
        })
    }
}

#[derive(Debug, Error, From, PartialEq)]
#[error("failed to parse bind")]
pub enum ParseBindError {
    #[error("invalid format")]
    InvalidFormat,
    #[error("{_0}")]
    ParseKeyError(ParseKeyError),
    #[error("{_0}")]
    ParseActionError(ParseActionError),
}

impl FromStr for Bind {
    type Err = ParseBindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Bind::parse(s.trim())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode as K, KeyModifiers as M};

    #[test]
    fn parse() {
        let strings = [
            ":: exit",
            "alt+::exit",
            " alt + : : exit ",
            ":::exit",
            "al+::exit",
            "alt+alt+::exit",
            "::invalid",
            ": : invalid",
        ];
        let parsed_strings = [
            Ok(Bind::new(Key::new(K::Char(':'), M::NONE), Action::Exit)),
            Ok(Bind::new(Key::new(K::Char(':'), M::ALT), Action::Exit)),
            Ok(Bind::new(Key::new(K::Char(':'), M::ALT), Action::Exit)),
            Err(ParseKeyError::InvalidKey("::".to_string()).into()),
            Err(ParseKeyError::InvalidModifier("al".to_string()).into()),
            Err(ParseKeyError::DuplicateModifier("alt".to_string()).into()),
            Err(ParseActionError("invalid".to_string()).into()),
            Err(ParseActionError("invalid".to_string()).into()),
        ];

        assert_eq!(strings.map(|s| s.parse()), parsed_strings);
    }
}
