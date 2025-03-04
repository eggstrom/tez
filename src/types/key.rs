use std::str::FromStr;

use crossterm::event::{KeyCode, KeyModifiers};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Key {
    key: KeyCode,
    modifiers: KeyModifiers,
}

impl Key {
    pub fn new(key: KeyCode, modifiers: KeyModifiers) -> Key {
        Key { key, modifiers }
    }

    fn parse(s: &str) -> Result<Self, ParseKeyError> {
        if s.starts_with('+') | s.ends_with('+') {
            Err(ParseKeyError::InvalidFormat)?;
        }

        let mut keys = s.split('+').map(|key| key.trim());
        let key = Key::parse_key(keys.next_back().ok_or(ParseKeyError::InvalidFormat)?)?;
        let mut modifiers = KeyModifiers::NONE;

        for key in keys {
            let modifier = Key::parse_modifier(key)?;
            if !modifiers.contains(modifier) {
                modifiers |= modifier;
            } else {
                Err(ParseKeyError::DuplicateModifier(key.to_string()))?;
            }
        }
        Ok(Key { key, modifiers })
    }

    fn parse_key(s: &str) -> Result<KeyCode, ParseKeyError> {
        Ok(match s {
            _ if s.is_empty() => Err(ParseKeyError::InvalidFormat)?,
            _ if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap()),
            _ if s.starts_with('f') => KeyCode::F(
                s[1..]
                    .parse()
                    .map_err(|_| ParseKeyError::InvalidKey(s.to_string()))?,
            ),
            "backspace" => KeyCode::Backspace,
            "enter" => KeyCode::Enter,
            "left" => KeyCode::Left,
            "right" => KeyCode::Right,
            "up" => KeyCode::Up,
            "down" => KeyCode::Down,
            "home" => KeyCode::Home,
            "end" => KeyCode::End,
            "page-up" => KeyCode::PageUp,
            "page-down" => KeyCode::PageDown,
            "tab" => KeyCode::Tab,
            "back-tab" => KeyCode::BackTab,
            "delete" => KeyCode::Delete,
            "insert" => KeyCode::Insert,
            "null" => KeyCode::Null,
            "escape" => KeyCode::Esc,
            _ => Err(ParseKeyError::InvalidKey(s.to_string()))?,
        })
    }

    fn parse_modifier(s: &str) -> Result<KeyModifiers, ParseKeyError> {
        Ok(match s {
            "shift" => KeyModifiers::SHIFT,
            "ctrl" => KeyModifiers::CONTROL,
            "alt" => KeyModifiers::ALT,
            _ => Err(ParseKeyError::InvalidModifier(s.to_string()))?,
        })
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("failed to parse key")]
pub enum ParseKeyError {
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid key: {_0}")]
    InvalidKey(String),
    #[error("invalid modifier: {_0}")]
    InvalidModifier(String),
    #[error("duplicate modifier: {_0}")]
    DuplicateModifier(String),
}

impl FromStr for Key {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Key::parse(s.trim())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode as K, KeyModifiers as M};

    #[test]
    fn parse() {
        let strings = [
            "",
            "a",
            "escape",
            "f0",
            "f255",
            "f256",
            "shift+a",
            "ctrl+a",
            "alt+a",
            "shift+ctrl+alt+a",
            " shift + ctrl + alt + a ",
            "a+",
            "+a",
            "shift",
            "shift+shift",
            "shif+a",
            "shift+shift+a",
        ];
        let parsed_strings = [
            Err(ParseKeyError::InvalidFormat),
            Ok(Key::new(K::Char('a'), M::NONE)),
            Ok(Key::new(K::Esc, M::NONE)),
            Ok(Key::new(K::F(0), M::NONE)),
            Ok(Key::new(K::F(255), M::NONE)),
            Err(ParseKeyError::InvalidKey("f256".to_string())),
            Ok(Key::new(K::Char('a'), M::SHIFT)),
            Ok(Key::new(K::Char('a'), M::CONTROL)),
            Ok(Key::new(K::Char('a'), M::ALT)),
            Ok(Key::new(K::Char('a'), M::from_bits(0b111).unwrap())),
            Ok(Key::new(K::Char('a'), M::from_bits(0b111).unwrap())),
            Err(ParseKeyError::InvalidFormat),
            Err(ParseKeyError::InvalidFormat),
            Err(ParseKeyError::InvalidKey("shift".to_string())),
            Err(ParseKeyError::InvalidKey("shift".to_string())),
            Err(ParseKeyError::InvalidModifier("shif".to_string())),
            Err(ParseKeyError::DuplicateModifier("shift".to_string())),
        ];

        assert_eq!(
            strings.iter().map(|s| s.parse()).collect::<Vec<_>>(),
            parsed_strings
        );
    }
}
