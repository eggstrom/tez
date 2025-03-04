use std::io;

use anyhow::{anyhow, bail, Result};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{DefaultTerminal, TerminalOptions};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task;

use crate::{config::Config, events::handle_events, state::State, tui::Tui, types::action::Action};

pub struct App<'a> {
    config: Config,
    state: State,
    tui: Tui<'a>,
    sender: UnboundedSender<Action>,
    receiver: UnboundedReceiver<Action>,
}

impl App<'_> {
    pub fn new(config: Config) -> Result<Self> {
        let (sender, receiver) = mpsc::unbounded_channel();
        let state = State::new()?;
        let tui = Tui::new(sender.clone())?;
        Ok(App {
            config,
            state,
            tui,
            sender,
            receiver,
        })
    }

    pub async fn run(mut self) -> Result<()> {
        let mut terminal = self.init_terminal()?;
        task::spawn(handle_events(self.sender.clone()));

        while self.state.running() {
            if self.state.should_draw() {
                self.draw(&mut terminal)?;
            }
            match self.receiver.recv().await.ok_or(anyhow!("todo"))? {
                Action::Error(error) => bail!(error),
                Action::Exit => self.state.exit(),
                Action::Draw => self.draw_forced(&mut terminal)?,
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
        if !self.config.is_inline() {
            execute!(io::stdout(), EnterAlternateScreen)?;
        } else {
            terminal.clear()?;
        }
        Ok(terminal)
    }

    fn restore_terminal(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        ratatui::restore();
        if !self.config.is_inline() {
            execute!(io::stdout(), LeaveAlternateScreen)?
        } else {
            terminal.clear()?;
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal
            .draw(|frame| frame.render_widget(&mut self.tui, self.config.area(frame.area())))?;
        Ok(())
    }

    fn draw_forced(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.draw(terminal)?;
        self.state.skip_frame();
        Ok(())
    }
}
