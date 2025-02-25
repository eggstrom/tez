use anyhow::Result;
use ratatui::{layout::Rect, Viewport};

use crate::utils::{Alignment, Extent};

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

    pub fn area(&self, frame_area: Rect) -> Rect {
        if self.width.is_none() {
            return frame_area;
        }

        let height = frame_area.height;
        let width = self
            .width
            .map_or(frame_area.width, |width| width.cells(frame_area.width))
            .min(frame_area.width);
        let x = match self.alignment {
            Alignment::Left => frame_area.x,
            Alignment::Center => frame_area.width / 2 - width / 2,
            Alignment::Right => frame_area.width - width,
            Alignment::Position(pos @ 0..) => {
                (pos.min(u16::MAX as i32) as u16).clamp(0, frame_area.width - width)
            }
            Alignment::Position(pos) => ((frame_area.width - width)
                .saturating_sub((pos.saturating_neg()).min(u16::MAX as i32) as u16))
            .clamp(0, frame_area.width - width),
        };

        Rect {
            x,
            y: frame_area.y,
            width,
            height,
        }
    }
}
