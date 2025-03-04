use std::{mem, str::FromStr};

use crossterm::event::KeyEvent;
use derive_more::From;
use thiserror::Error;

#[derive(Debug, From)]
pub enum Action {
    Error(anyhow::Error),
    Exit,
    Draw,
    Tui(TuiAction),
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Action::Error(_), Action::Error(_)) => true,
            (Action::Error(_), _) | (_, Action::Error(_)) => false,
            (Action::Tui(a), Action::Tui(b)) => a == b,
            _ => mem::discriminant(self) == mem::discriminant(other),
        }
    }
}

#[derive(Debug, PartialEq)]
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
