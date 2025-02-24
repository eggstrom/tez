use anyhow::Result;
use crossterm::{cursor, terminal};
use ratatui::{layout::Rect, Viewport};

use crate::utils::Extent;

pub struct Config {
    pub width: Option<Extent>,
    pub height: Option<Extent>,
    pub x: Option<Extent>,
    pub y: Option<Extent>,
}

impl Config {
    pub fn viewport(&self) -> Result<Viewport> {
        let (term_w, term_h) = terminal::size()?;
        let (cur_x, cur_y) = cursor::position().unwrap_or((0, 0));

        Ok(match (self.height, self.width, self.x, self.y) {
            (None, None, None, None) => Viewport::Fullscreen,
            (Some(h), None, None, None) => Viewport::Inline(h.cells(term_h)),
            _ => Viewport::Fixed(Rect::new(
                self.x.map_or(cur_x, |x| x.cells(term_w)),
                self.y.map_or(cur_y, |y| y.cells(term_h)),
                self.width.map_or(term_w, |w| w.cells(term_w)),
                self.height.map_or(term_h, |h| h.cells(term_h)),
            )),
        })
    }
}
