use std::sync::mpsc::{self, Receiver};

use anyhow::{bail, Result};

use crate::{
    actions::{handle_events, Action},
    tui::Tui,
};

pub struct App<'a> {
    receiver: Receiver<Action>,
    tui: Tui<'a>,
}

impl Default for App<'_> {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        handle_events(sender);
        App {
            receiver,
            tui: Tui::default(),
        }
    }
}

impl App<'_> {
    pub fn run(mut self) -> Result<()> {
        let mut terminal = ratatui::init();
        loop {
            terminal.draw(|frame| frame.render_widget(&mut self.tui, frame.area()))?;
            match self.receiver.recv()? {
                Action::Error(error) => bail!(error),
                Action::Quit => break,
                Action::Tui(action) => self.tui.handle_action(action),
            }
        }
        ratatui::restore();
        Ok(())
    }
}
