pub struct State {
    running: bool,
}

impl Default for State {
    fn default() -> Self {
        State { running: true }
    }
}

impl State {
    pub fn running(&self) -> bool {
        self.running
    }

    pub fn exit(&mut self) {
        self.running = false;
    }
}
