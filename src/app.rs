use std::sync::mpsc;

use anyhow::{bail, Result};
use ratatui::{widgets::Clear, DefaultTerminal};

use crate::{
    actions::{handle_events, Action},
    config::Config,
    state::State,
    tui::Tui,
};

pub struct App<'a> {
    config: Config,
    state: State,
    tui: Tui<'a>,
}

impl App<'_> {
    pub fn new(config: Config) -> Result<Self> {
        Ok(App {
            config,
            state: State::default(),
            tui: Tui::new()?,
        })
    }

    pub fn run(mut self) -> Result<()> {
        let mut terminal = ratatui::init_with_options(self.config.terminal_options()?);
        let (sender, receiver) = mpsc::channel();
        handle_events(sender);

        while self.state.running() {
            self.draw(&mut terminal)?;
            match receiver.recv()? {
                Action::Error(error) => bail!(error),
                Action::Exit => self.exit(&mut terminal)?,
                Action::Tui(action) => self.tui.handle_action(action),
            }
        }

        ratatui::restore();
        Ok(())
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(&mut self.tui, frame.area()))?;
        Ok(())
    }

    fn exit(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(Clear, frame.area()))?;
        self.state.exit();
        Ok(())
    }
}
