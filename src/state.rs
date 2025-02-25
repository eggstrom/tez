use anyhow::Result;
use crossterm::terminal;

pub struct State {
    terminal_size: (u16, u16),
    running: bool,
}

impl State {
    pub fn new() -> Result<Self> {
        Ok(State {
            terminal_size: terminal::size()?,
            running: true,
        })
    }

    pub fn terminal_size(&self) -> (u16, u16) {
        self.terminal_size
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn exit(&mut self) {
        self.running = false;
    }
}
