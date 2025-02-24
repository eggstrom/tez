use std::{io, sync::mpsc};

use anyhow::{bail, Result};
use crossterm::{
    cursor::{RestorePosition, SavePosition},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{widgets::Clear, DefaultTerminal, TerminalOptions, Viewport};

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
                Action::Exit => self.exit(&mut terminal)?,
                Action::Tui(action) => self.tui.handle_action(action),
            }
        }

        self.restore_terminal()?;
        Ok(())
    }

    fn init_terminal(&self) -> Result<DefaultTerminal> {
        execute!(io::stdout(), SavePosition)?;
        let viewport = self.config.viewport()?;
        if let Viewport::Fullscreen = viewport {
            execute!(io::stdout(), EnterAlternateScreen)?;
        }
        Ok(ratatui::init_with_options(TerminalOptions { viewport }))
    }

    fn restore_terminal(&self) -> Result<()> {
        ratatui::restore();
        let viewport = self.config.viewport()?;
        if let Viewport::Fullscreen = viewport {
            execute!(io::stdout(), LeaveAlternateScreen)?;
        }
        execute!(io::stdout(), RestorePosition)?;
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
