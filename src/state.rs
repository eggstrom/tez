use anyhow::Result;
use ratatui::Viewport;

use crate::config::Config;

pub struct State {
    viewport: Viewport,
    running: bool,
}

impl State {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(State {
            viewport: config.viewport()?,
            running: true,
        })
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn exit(&mut self) {
        self.running = false;
    }
}
