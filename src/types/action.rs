use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use derive_more::From;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use thiserror::Error;

use super::key::Key;

#[derive(Clone, Debug, From, PartialEq)]
pub enum Action {
    Exit,
    Draw,
    Tui(TuiAction),
}

#[derive(Clone, Debug, From, PartialEq)]
pub enum TuiAction {
    Next,
    Previous,
    First,
    Last,
    Input(InputAction),
}

#[derive(Clone, Debug, From, PartialEq)]
pub enum InputAction {
    Key(Key),
    MoveForward,
    MoveBack,
    MoveUp,
    MoveDown,
    MoveForwardWord,
    MoveBackWord,
    MoveToEndOfWord,
    MoveToTop,
    MoveToBottom,
    MoveToHead,
    MoveToEnd,
    Delete,
    DeleteNext,
    DeleteWord,
    DeleteNextWord,
    DeleteToHead,
    DeleteToEnd,
}

impl Action {
    fn parse(s: &str) -> Result<Self, ParseActionError> {
        Ok(match s {
            "exit" => Action::Exit,
            "next" => TuiAction::Next.into(),
            "previous" => TuiAction::Previous.into(),
            "first" => TuiAction::First.into(),
            "last" => TuiAction::Last.into(),
            "move-forward" => InputAction::MoveForward.into(),
            "move-back" => InputAction::MoveBack.into(),
            "move-up" => InputAction::MoveUp.into(),
            "move-down" => InputAction::MoveDown.into(),
            "move-forward-word" => InputAction::MoveForwardWord.into(),
            "move-back-word" => InputAction::MoveBackWord.into(),
            "move-to-end-of-word" => InputAction::MoveToEndOfWord.into(),
            "move-to-top" => InputAction::MoveToTop.into(),
            "move-to-bottom" => InputAction::MoveToBottom.into(),
            "move-to-head" => InputAction::MoveToHead.into(),
            "move-to-end" => InputAction::MoveToEnd.into(),
            "delete" => InputAction::Delete.into(),
            "delete-next" => InputAction::DeleteNext.into(),
            "delete-word" => InputAction::DeleteWord.into(),
            "delete-next-word" => InputAction::DeleteNextWord.into(),
            "delete-to-head" => InputAction::DeleteToHead.into(),
            "delete-to-end" => InputAction::DeleteToEnd.into(),
            _ => Err(ParseActionError(s.to_string()))?,
        })
    }
}

impl From<InputAction> for Action {
    fn from(value: InputAction) -> Self {
        Action::Tui(TuiAction::Input(value))
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
