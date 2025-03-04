use std::io;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{DefaultTerminal, TerminalOptions};
use tokio::sync::{mpsc, watch};
use tokio::task;

use crate::{
    config::Config, events::handle_events, searcher::debounce_draws, state::State, tui::Tui,
    types::action::Action,
};

pub struct App<'a> {
    config: Config,
    state: State,
    tui: Tui<'a>,
}

impl App<'_> {
    pub async fn run(config: Config) -> Result<()> {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let (draw_sender, draw_receiver) = watch::channel(());

        let state = State::new()?;
        let tui = Tui::new(draw_sender)?;
        let mut app = App { config, state, tui };

        let mut terminal = app.init_terminal()?;
        task::spawn(handle_events(sender.clone()));
        task::spawn(debounce_draws(draw_receiver, sender));

        while app.state.running() {
            if app.state.should_draw() {
                app.draw(&mut terminal)?;
            }
            match receiver.recv().await {
                Some(Action::Error(error)) => Err(error)?,
                Some(Action::Exit) => app.state.exit(),
                Some(Action::Draw) => app.draw_forced(&mut terminal)?,
                Some(Action::Tui(action)) => app.tui.handle_action(action),
                None => break,
            }
        }

        app.restore_terminal(&mut terminal)?;
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

    fn restore_terminal(&self, terminal: &mut DefaultTerminal) -> Result<()> {
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
