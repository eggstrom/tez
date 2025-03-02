use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use anyhow::Result;

use thiserror::Error;

use super::extent::Extent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Left(Extent),
    Center,
    Right(Extent),
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left(Extent::ZERO)
    }
}

impl Alignment {
    fn parse(s: &str) -> Result<Self, ParseAlignmentError> {
        Ok(if s == "center" {
            Alignment::Center
        } else if let Some(s) = s.strip_prefix("left") {
            if s.is_empty() {
                Alignment::Left(Extent::ZERO)
            } else {
                Alignment::Left(Alignment::parse_offset(s.trim_start())?)
            }
        } else if let Some(s) = s.strip_prefix("right") {
            if s.is_empty() {
                Alignment::Right(Extent::ZERO)
            } else {
                Alignment::Right(Alignment::parse_offset(s.trim_start())?)
            }
        } else {
            Err(ParseAlignmentError)?
        })
    }

    fn parse_offset(s: &str) -> Result<Extent, ParseAlignmentError> {
        s.strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.trim().parse().ok())
            .ok_or(ParseAlignmentError)
    }
}

impl Display for Alignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Alignment::Left(Extent::Cells(0)) => "left".fmt(f),
            Alignment::Left(e) => write!(f, "left({e})"),
            Alignment::Center => "center".fmt(f),
            Alignment::Right(Extent::Cells(0)) => "right".fmt(f),
            Alignment::Right(e) => write!(f, "right({e})"),
        }
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("failed to parse alignment")]
pub struct ParseAlignmentError;

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
        assert_eq!("left".parse(), Ok(Alignment::Left(Extent::ZERO)));
        assert_eq!("left(1)".parse(), Ok(Alignment::Left(Extent::Cells(1))));
        assert_eq!("left ( 1 )".parse(), Ok(Alignment::Left(Extent::Cells(1))));
        assert!(" left(1) ".parse::<Alignment>().is_err());

        assert_eq!("center".parse(), Ok(Alignment::Center));
        assert!(" center ".parse::<Alignment>().is_err());

        assert_eq!("right".parse(), Ok(Alignment::Right(Extent::ZERO)));
        assert_eq!("right(1)".parse(), Ok(Alignment::Right(Extent::Cells(1))));
        assert_eq!(
            "right ( 1 )".parse(),
            Ok(Alignment::Right(Extent::Cells(1)))
        );
        assert!(" right(1) ".parse::<Alignment>().is_err());
    }
}
