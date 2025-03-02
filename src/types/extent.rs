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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Extent {
    Cells(u16),
    Percentage(f32),
}

impl Extent {
    pub const ZERO: Extent = Extent::Cells(0);

    pub fn cells(&self, size: u16) -> u16 {
        match self {
            Extent::Cells(c) => *c,
            Extent::Percentage(p) => (p * size as f32) as u16,
        }
    }
}

impl Display for Extent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Extent::Cells(c) => write!(f, "{c}"),
            Extent::Percentage(p) => write!(f, "{}%", p * 100.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("failed to parse extent")]
pub struct ParseExtentError;

impl FromStr for Extent {
    type Err = ParseExtentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_suffix('%') {
            s.trim_end()
                .parse()
                .map(|float: f32| Extent::Percentage(float / 100.0))
                .map_err(|_| ParseExtentError)
        } else {
            s.parse().map(Extent::Cells).map_err(|_| ParseExtentError)
        }
    }
}

impl<'de> Deserialize<'de> for Extent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ExtentVisitor)
    }
}

struct ExtentVisitor;

impl Visitor<'_> for ExtentVisitor {
    type Value = Extent;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a number of cells between 0 and 65535 or a percentage")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Extent::Cells(
            value
                .try_into()
                .map_err(|_| de::Error::custom(ParseExtentError))?,
        ))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRINGS: &[&str] = &["1", "10%", "10.0%", "10 %", "10.0", " 1 ", " 10% "];

    const PARSED_STRINGS: &[Result<Extent, ParseExtentError>] = &[
        Ok(Extent::Cells(1)),
        Ok(Extent::Percentage(0.1)),
        Ok(Extent::Percentage(0.1)),
        Ok(Extent::Percentage(0.1)),
        Err(ParseExtentError),
        Err(ParseExtentError),
        Err(ParseExtentError),
    ];

    #[test]
    fn parse() {
        assert_eq!(
            STRINGS.iter().map(|s| s.parse()).collect::<Vec<_>>(),
            PARSED_STRINGS
        );
    }

    const INTS: &[i64] = &[0, -1, 65536];

    const DESERIALIZED_INTS: &[Result<Extent, ParseExtentError>] = &[
        Ok(Extent::Cells(0)),
        Err(ParseExtentError),
        Err(ParseExtentError),
    ];

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

        assert_eq!(
            INTS.iter()
                .map(|int| toml::Value::Integer(*int).try_into())
                .collect::<Vec<_>>(),
            DESERIALIZED_INTS
                .iter()
                .map(|res| res.map_err(<toml::de::Error as de::Error>::custom))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn convert_to_cells() {
        assert_eq!(Extent::Cells(10).cells(100), 10);
        assert_eq!(Extent::Percentage(0.5).cells(100), 50);
        assert_eq!(Extent::Percentage(1.0).cells(100), 100);
        assert_eq!(Extent::Percentage(1.5).cells(100), 150);
    }
}
