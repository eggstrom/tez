use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use anyhow::Result;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
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
            Alignment::Left(if s.is_empty() {
                Extent::ZERO
            } else {
                Alignment::parse_offset(s.trim_start())?
            })
        } else if let Some(s) = s.strip_prefix("right") {
            Alignment::Right(if s.is_empty() {
                Extent::ZERO
            } else {
                Alignment::parse_offset(s.trim_start())?
            })
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

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("failed to parse alignment")]
pub struct ParseAlignmentError;

impl FromStr for Alignment {
    type Err = ParseAlignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Alignment::parse(s)
    }
}

impl<'de> Deserialize<'de> for Alignment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AlignmentVisitor)
    }
}

struct AlignmentVisitor;

impl Visitor<'_> for AlignmentVisitor {
    type Value = Alignment;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        "an alignment (left[(<EXTENT>)] | center | right[(<EXTENT>)])".fmt(formatter)
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

    const STRINGS: &[&str] = &[
        "left",
        "left(1)",
        "left ( 1 )",
        " left ",
        "center",
        " center ",
        "right",
        "right(1)",
        "right ( 1 )",
        " right ",
    ];
    const PARSED_STRINGS: &[Result<Alignment, ParseAlignmentError>] = &[
        Ok(Alignment::Left(Extent::ZERO)),
        Ok(Alignment::Left(Extent::Cells(1))),
        Ok(Alignment::Left(Extent::Cells(1))),
        Err(ParseAlignmentError),
        Ok(Alignment::Center),
        Err(ParseAlignmentError),
        Ok(Alignment::Right(Extent::ZERO)),
        Ok(Alignment::Right(Extent::Cells(1))),
        Ok(Alignment::Right(Extent::Cells(1))),
        Err(ParseAlignmentError),
    ];

    #[test]
    fn parse() {
        assert_eq!(
            STRINGS.iter().map(|s| s.parse()).collect::<Vec<_>>(),
            PARSED_STRINGS
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            STRINGS
                .iter()
                .map(|s| toml::Value::String(s.to_string()).try_into())
                .collect::<Vec<_>>(),
            PARSED_STRINGS
                .iter()
                .map(|res| res.map_err(<toml::de::Error as de::Error>::custom))
                .collect::<Vec<_>>()
        );
    }
}
