use anyhow::Result;

use crate::config::Config;

pub struct State {
    running: bool,
}

impl State {
    pub fn new(_: &Config) -> Result<Self> {
        Ok(State { running: true })
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn exit(&mut self) {
        self.running = false;
    }
}
