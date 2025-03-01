use anyhow::Result;
use ratatui::{layout::Rect, Viewport};

use crate::utils::{alignment::Alignment, extent::Extent};

pub struct Config {
    pub width: Option<Extent>,
    pub height: Option<Extent>,
    pub alignment: Alignment,
}

impl Config {
    pub fn is_inline(&self) -> bool {
        self.height.is_some()
    }

    pub fn viewport(&self, term_height: u16) -> Result<Viewport> {
        Ok(match self.height {
            None => Viewport::Fullscreen,
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
        let x = match self.alignment {
            Alignment::Left(pos) => pos.min(area.width - width),
            Alignment::Center => area.width / 2 - width / 2,
            Alignment::Right(pos) => area.width.saturating_sub(width).saturating_sub(pos),
        };

        Rect {
            x,
            y: area.y,
            width,
            height,
        }
    }
}
