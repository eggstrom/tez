use std::{io, sync::mpsc};

use anyhow::{bail, Result};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{DefaultTerminal, TerminalOptions};

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
            state: State::new()?,
            tui: Tui::new()?,
        })
    }

    pub fn run(mut self) -> Result<()> {
        let (sender, receiver) = mpsc::channel();
        let mut terminal = self.init_terminal()?;
        handle_events(sender);

        while self.state.running() {
            self.draw(&mut terminal)?;
            match receiver.recv()? {
                Action::Error(error) => bail!(error),
                Action::Exit => self.state.exit(),
                Action::Tui(action) => self.tui.handle_action(action),
            }
        }

        self.restore_terminal(&mut terminal)?;
        Ok(())
    }

    fn init_terminal(&mut self) -> Result<DefaultTerminal> {
        let mut terminal = ratatui::init_with_options(TerminalOptions {
            viewport: self.config.viewport(self.state.terminal_size().1)?,
        });

        match self.config.is_inline() {
            false => execute!(io::stdout(), EnterAlternateScreen)?,
            true => terminal.clear()?,
        }
        Ok(terminal)
    }

    fn restore_terminal(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        ratatui::restore();
        match self.config.is_inline() {
            false => execute!(io::stdout(), LeaveAlternateScreen)?,
            true => terminal.clear()?,
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal
            .draw(|frame| frame.render_widget(&mut self.tui, self.config.area(frame.area())))?;
        Ok(())
    }
}
