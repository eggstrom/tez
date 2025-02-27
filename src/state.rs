use anyhow::Result;
use crossterm::terminal;

pub struct State {
    terminal_size: (u16, u16),
    running: bool,
    skip_frame: bool,
}

impl State {
    pub fn new() -> Result<Self> {
        Ok(State {
            terminal_size: terminal::size()?,
            running: true,
            skip_frame: false,
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

    pub fn skip_frame(&mut self) {
        self.skip_frame = true;
    }

    pub fn should_draw(&mut self) -> bool {
        let should_draw = !self.skip_frame;
        self.skip_frame = false;
        should_draw
    }
}
