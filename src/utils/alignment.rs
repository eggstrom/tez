use std::str::FromStr;

use anyhow::Result;
use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Display, Default, Clone, Copy, PartialEq)]
pub enum Alignment {
    #[default]
    #[display("left")]
    Left,
    #[display("center")]
    Center,
    #[display("right")]
    Right,
    #[display("{_0}")]
    Position(i32),
}

#[derive(Debug, Error, PartialEq)]
#[error("failed to parse alignment")]
pub struct ParseAlignmentError;

impl FromStr for Alignment {
    type Err = ParseAlignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "left" => Alignment::Left,
            "center" => Alignment::Center,
            "right" => Alignment::Right,
            s => s
                .parse()
                .map(Alignment::Position)
                .map_err(|_| ParseAlignmentError)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_alignment() {
        assert_eq!("left".parse(), Ok(Alignment::Left));
        assert_eq!("center".parse(), Ok(Alignment::Center));
        assert_eq!("right".parse(), Ok(Alignment::Right));
        assert_eq!("1".parse(), Ok(Alignment::Position(1)));
        assert_eq!("-1".parse(), Ok(Alignment::Position(-1)));
    }
}
