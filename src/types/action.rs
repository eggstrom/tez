use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crossterm::event::KeyEvent;
use derive_more::From;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use thiserror::Error;

#[derive(Clone, Debug, From, PartialEq)]
pub enum Action {
    Exit,
    Draw,
    Tui(TuiAction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TuiAction {
    First,
    Last,
    Next,
    Previous,
    Key(KeyEvent),
}

impl Action {
    fn parse(s: &str) -> Result<Self, ParseActionError> {
        Ok(match s {
            "exit" => Action::Exit,
            "first" => TuiAction::First.into(),
            "last" => TuiAction::Last.into(),
            "next" => TuiAction::Next.into(),
            "previous" => TuiAction::Previous.into(),
            _ => Err(ParseActionError(s.to_string()))?,
        })
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("failed to parse action")]
pub struct ParseActionError(pub String);

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Action::parse(s.trim())
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ActionVisitor)
    }
}

struct ActionVisitor;

impl Visitor<'_> for ActionVisitor {
    type Value = Action;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        "an action".fmt(formatter)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse().map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let strings = ["exit", "invalid"];
        let parsed_strings = [
            Ok(Action::Exit),
            Err(<toml::de::Error as de::Error>::custom(ParseActionError(
                "invalid".to_string(),
            ))),
        ];

        assert_eq!(
            strings.map(|s| toml::Value::String(s.to_string()).try_into()),
            parsed_strings
        );
    }
}
