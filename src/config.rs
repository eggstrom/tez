use anyhow::Result;
use crossterm::terminal;
use ratatui::{TerminalOptions, Viewport};

use crate::utils::Extent;

pub struct Config {
    pub height: Option<Extent>,
}

impl Config {
    pub fn terminal_options(&self) -> Result<TerminalOptions> {
        let (_, height) = terminal::size()?;
        let viewport = match self.height {
            None => Viewport::Fullscreen,
            Some(h) => Viewport::Inline(h.cells(height)),
        };
        Ok(TerminalOptions { viewport })
    }
}
