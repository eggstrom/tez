use std::{
    fmt::{self, Formatter},
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
            value.try_into().map_err(|e| de::Error::custom(e))?,
        ))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(|e| de::Error::custom(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!("1".parse(), Ok(Extent::Cells(1)));
        assert_eq!("10%".parse(), Ok(Extent::Percentage(0.1)));
        assert_eq!("10.0%".parse(), Ok(Extent::Percentage(0.1)));
        assert_eq!("10 %".parse::<Extent>(), Err(ParseExtentError));
        assert_eq!("10.0".parse::<Extent>(), Err(ParseExtentError));
        assert_eq!("a".parse::<Extent>(), Err(ParseExtentError));
    }

    #[test]
    fn deserialize() {
        assert_eq!(toml::Value::Integer(1).try_into(), Ok(Extent::Cells(1)));
        assert_eq!(
            toml::Value::String("1".to_string()).try_into(),
            Ok(Extent::Cells(1))
        );
        assert_eq!(
            toml::Value::String("1%".to_string()).try_into(),
            Ok(Extent::Percentage(0.01))
        );

        assert!(toml::Value::Integer(-1).try_into::<Extent>().is_err());
        assert!(toml::Value::Integer(i64::MAX).try_into::<Extent>().is_err());
        assert!(toml::Value::String((-1).to_string())
            .try_into::<Extent>()
            .is_err());
        assert!(toml::Value::String((i64::MAX).to_string())
            .try_into::<Extent>()
            .is_err());
    }

    #[test]
    fn convert_to_cells() {
        assert_eq!(Extent::Cells(10).cells(100), 10);
        assert_eq!(Extent::Percentage(0.5).cells(100), 50);
        assert_eq!(Extent::Percentage(1.0).cells(100), 100);
        assert_eq!(Extent::Percentage(1.5).cells(100), 150);
    }
}
