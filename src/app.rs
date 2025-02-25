use std::{io, sync::mpsc};

use anyhow::{bail, Result};
use crossterm::{
    cursor::{RestorePosition, SavePosition},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{DefaultTerminal, TerminalOptions, Viewport};

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
        let state = State::new(&config)?;
        Ok(App {
            config,
            state,
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
            viewport: self.state.viewport().clone(),
        });
        match self.state.viewport() {
            Viewport::Fullscreen => execute!(io::stdout(), EnterAlternateScreen)?,
            Viewport::Inline(_) => terminal.clear()?,
            // TODO: Make this clear viewport before drawing
            Viewport::Fixed(..) => execute!(io::stdout(), SavePosition)?,
        }
        Ok(terminal)
    }

    fn restore_terminal(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        ratatui::restore();
        match self.state.viewport() {
            Viewport::Fullscreen => execute!(io::stdout(), LeaveAlternateScreen)?,
            Viewport::Inline(_) => terminal.clear()?,
            // TODO: Make this restore viewport content
            Viewport::Fixed(..) => execute!(io::stdout(), RestorePosition)?,
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(&mut self.tui, frame.area()))?;
        Ok(())
    }
}
