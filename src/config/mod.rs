use std::{fs, path::Path};

use anyhow::Result;
use binds::Binds;
use clap::Parser;
use cli::Cli;
use ratatui::{layout::Rect, Viewport};
use serde::Deserialize;

use crate::types::{action::Action, alignment::Alignment, extent::Extent, key::Key};

mod binds;
mod cli;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    width: Option<Extent>,
    height: Option<Extent>,
    alignment: Option<Alignment>,

    #[serde(default)]
    binds: Binds,
    #[serde(default)]
    disable_default_binds: bool,
}

impl Config {
    pub fn load() -> Result<Self> {
        let cli = Cli::parse();
        let mut config = cli
            .config_path()
            .map(Config::parse)
            .unwrap_or(Ok(Config::default()))?;
        config = config.overwrite(&cli.config());
        config.insert_default_binds();
        Ok(config)
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

    pub fn set_disable_default_binds(&mut self, disable_default_binds: bool) {
        self.disable_default_binds = disable_default_binds;
    }

    #[allow(clippy::option_map_unit_fn)]
    fn overwrite(&self, other: &Self) -> Self {
        let mut new = self.clone();
        other.width.map(|w| new.set_width(Some(w)));
        other.height.map(|w| new.set_height(Some(w)));
        other.alignment.map(|w| new.set_alignment(Some(w)));
        new.binds.overwrite(&other.binds);
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

    pub fn insert_default_binds(&mut self) {
        if !self.disable_default_binds {
            self.binds.insert_defaults();
        }
    }

    pub fn action_for_key(&self, key: &Key) -> Option<Action> {
        self.binds.action_for_key(key)
    }
}
