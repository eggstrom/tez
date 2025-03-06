use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use clap::Parser;
use derive_more::From;
use ratatui::{layout::Rect, Viewport};
use serde::Deserialize;

use crate::{
    cli::Cli,
    types::{action::Action, alignment::Alignment, extent::Extent, key::Key},
};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    width: Option<Extent>,
    height: Option<Extent>,
    alignment: Option<Alignment>,
    #[serde(default)]
    binds: Binds,
}

#[derive(Clone, Debug, Default, Deserialize, From, PartialEq)]
pub struct Binds(HashMap<Key, Action>);

impl Config {
    pub fn load() -> Result<Self> {
        let cli = Cli::parse();
        let config = cli
            .config_path()
            .map(Config::parse)
            .unwrap_or(Ok(Config::default()))?;
        Ok(config.overwrite(&cli.config()))
    }

    fn parse<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn set_width(&mut self, width: Option<Extent>) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: Option<Extent>) {
        self.height = height;
    }

    pub fn set_alignment(&mut self, alignment: Option<Alignment>) {
        self.alignment = alignment;
    }

    #[allow(clippy::option_map_unit_fn)]
    fn overwrite(&self, other: &Self) -> Self {
        let mut new = self.clone();
        other.width.map(|w| new.set_width(Some(w)));
        other.height.map(|w| new.set_height(Some(w)));
        other.alignment.map(|w| new.set_alignment(Some(w)));
        new
    }

    pub fn is_inline(&self) -> bool {
        self.height.is_some()
    }

    pub fn viewport(&self, term_height: u16) -> Result<Viewport> {
        Ok(match self.height {
            None => Viewport::Fullscreen,
            // Viewport won't clear properly if inline height isn't below 100%
            Some(height) => Viewport::Inline(height.cells(term_height).min(term_height - 1)),
        })
    }

    pub fn area(&self, area: Rect) -> Rect {
        if self.width.is_none() {
            return area;
        }

        let height = area.height;
        let width = self
            .width
            .map_or(area.width, |width| width.cells(area.width))
            .min(area.width);
        let x = match self.alignment.unwrap_or_default() {
            Alignment::Left(offset) => offset.cells(area.width).min(area.width - width),
            Alignment::Center => area.width / 2 - width / 2,
            Alignment::Right(offset) => area
                .width
                .saturating_sub(width)
                .saturating_sub(offset.cells(area.width)),
        };

        Rect {
            x,
            y: area.y,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode as K, KeyModifiers as M};

    #[test]
    fn binds() {
        let parsed = toml::from_str::<Binds>("'ctrl+c' = 'exit'");
        assert_eq!(
            parsed,
            Ok(HashMap::from([(Key::new(K::Char('c'), M::CONTROL), Action::Exit)]).into())
        );
    }
}
