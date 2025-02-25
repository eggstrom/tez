use std::str::FromStr;

use anyhow::Result;
use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Extent {
    Cells(u16),
    Percentage(f32),
}

impl Extent {
    pub fn cells(&self, size: u16) -> u16 {
        match self {
            Extent::Cells(c) => *c,
            Extent::Percentage(p) => (p * size as f32) as u16,
        }
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("failed to parse extent")]
pub struct ParseExtentError;

impl FromStr for Extent {
    type Err = ParseExtentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.ends_with('%') {
            s.parse().map(Extent::Cells).map_err(|_| ParseExtentError)
        } else {
            s[0..s.len() - 1]
                .parse()
                .map(|float: f32| Extent::Percentage(float / 100.0))
                .map_err(|_| ParseExtentError)
        }
    }
}

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
    fn parse_extent() {
        assert_eq!("1".parse(), Ok(Extent::Cells(1)));
        assert_eq!("10%".parse(), Ok(Extent::Percentage(0.1)));
        assert_eq!("10.0%".parse(), Ok(Extent::Percentage(0.1)));
        assert_eq!("10 %".parse::<Extent>(), Err(ParseExtentError));
        assert_eq!("10.0".parse::<Extent>(), Err(ParseExtentError));
        assert_eq!("a".parse::<Extent>(), Err(ParseExtentError));
    }

    #[test]
    fn convert_extent_to_cells() {
        assert_eq!(Extent::Cells(10).cells(100), 10);
        assert_eq!(Extent::Percentage(0.5).cells(100), 50);
        assert_eq!(Extent::Percentage(1.0).cells(100), 100);
        assert_eq!(Extent::Percentage(1.5).cells(100), 150);
    }

    #[test]
    fn parse_alignment() {
        assert_eq!("left".parse(), Ok(Alignment::Left));
        assert_eq!("center".parse(), Ok(Alignment::Center));
        assert_eq!("right".parse(), Ok(Alignment::Right));
        assert_eq!("1".parse(), Ok(Alignment::Position(1)));
        assert_eq!("-1".parse(), Ok(Alignment::Position(-1)));
    }
}
