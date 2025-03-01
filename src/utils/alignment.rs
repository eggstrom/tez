use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use anyhow::Result;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Left(u16),
    Center,
    Right(u16),
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left(0)
    }
}

impl Display for Alignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Alignment::Left(0) => "left".fmt(f),
            Alignment::Left(pos) => write!(f, "left({pos})"),
            Alignment::Center => "center".fmt(f),
            Alignment::Right(0) => "right".fmt(f),
            Alignment::Right(pos) => write!(f, "right({pos})"),
        }
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("failed to parse alignment")]
pub struct ParseAlignmentError;

impl Alignment {
    fn parse(s: &str) -> Result<Self, ParseAlignmentError> {
        Ok(if s == "center" {
            Alignment::Center
        } else if let Some(s) = s.strip_prefix("left") {
            if s.is_empty() {
                Alignment::Left(0)
            } else {
                Alignment::Left(Alignment::parse_offset(s.trim_start())?)
            }
        } else if let Some(s) = s.strip_prefix("right") {
            if s.is_empty() {
                Alignment::Right(0)
            } else {
                Alignment::Right(Alignment::parse_offset(s.trim_start())?)
            }
        } else {
            Err(ParseAlignmentError)?
        })
    }

    fn parse_offset(s: &str) -> Result<u16, ParseAlignmentError> {
        s.strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.trim().parse().ok())
            .ok_or(ParseAlignmentError)
    }
}

impl FromStr for Alignment {
    type Err = ParseAlignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Alignment::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!("left".parse(), Ok(Alignment::Left(0)));
        assert_eq!("left(1)".parse(), Ok(Alignment::Left(1)));
        assert_eq!("left ( 1 )".parse(), Ok(Alignment::Left(1)));
        assert!(" left(1) ".parse::<Alignment>().is_err());

        assert_eq!("center".parse(), Ok(Alignment::Center));
        assert!(" center ".parse::<Alignment>().is_err());

        assert_eq!("right".parse(), Ok(Alignment::Right(0)));
        assert_eq!("right(1)".parse(), Ok(Alignment::Right(1)));
        assert_eq!("right ( 1 )".parse(), Ok(Alignment::Right(1)));
        assert!(" right(1) ".parse::<Alignment>().is_err());
    }
}
