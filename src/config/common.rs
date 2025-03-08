use anyhow::Result;
use clap::Args;
use ratatui::{layout::Rect, Viewport};
use serde::Deserialize;

use crate::types::{alignment::Alignment, extent::Extent};

#[derive(Args, Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CommonConfig {
    /// Disable default binds
    #[arg(short, long)]
    #[serde(default)]
    pub disable_default_binds: bool,

    /// Set viewport width
    #[arg(short = 'W', long)]
    width: Option<Extent>,
    /// Set viewport height
    #[arg(short = 'H', long)]
    height: Option<Extent>,
    /// Set viewport alignment
    #[arg(short = 'A', long)]
    alignment: Option<Alignment>,
}

impl CommonConfig {
    #[allow(clippy::option_map_unit_fn)]
    pub fn overwrite(&mut self, other: &Self) {
        self.disable_default_binds = other.disable_default_binds;
        other.width.map(|w| self.width = Some(w));
        other.height.map(|h| self.height = Some(h));
        other.alignment.map(|a| self.alignment = Some(a));
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

    #[test]
    fn overwrite() {
        let a = CommonConfig::default();
        let b = CommonConfig {
            disable_default_binds: true,
            height: Some(Extent::ZERO),
            width: Some(Extent::ZERO),
            alignment: Some(Alignment::default()),
        };
        let mut c = a.clone();
        c.overwrite(&b);

        assert_eq!(b, c);
    }
}
