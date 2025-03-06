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
    config::Config,
    events::{handle_events, Message},
    searcher::debounce_draws,
    state::State,
    tui::Tui,
    types::{action::Action, key::Key},
};

pub struct App<'a> {
    config: Config,
    state: State,
    tui: Tui<'a>,
    terminal: DefaultTerminal,
}

impl App<'_> {
    pub async fn run() -> Result<()> {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let (draw_sender, draw_receiver) = watch::channel(());

        let config = Config::load()?;
        let state = State::new()?;
        let tui = Tui::new(draw_sender)?;
        let terminal = App::init_terminal(&config, &state)?;
        let mut app = App {
            config,
            state,
            tui,
            terminal,
        };

        task::spawn(handle_events(sender.clone()));
        task::spawn(debounce_draws(draw_receiver, sender));

        while app.state.running() {
            if app.state.should_draw() {
                app.draw()?;
            }
            match receiver.recv().await {
                Some(Message::Error(error)) => Err(error)?,
                Some(Message::Action(action)) => app.handle_action(action)?,
                Some(Message::Key(key)) => app.handle_key(key)?,
                None => break,
            }
        }

        app.restore_terminal()?;
        Ok(())
    }

    fn init_terminal(config: &Config, state: &State) -> Result<DefaultTerminal> {
        let mut terminal = ratatui::init_with_options(TerminalOptions {
            viewport: config.viewport(state.terminal_size().1)?,
        });
        if !config.is_inline() {
            execute!(io::stdout(), EnterAlternateScreen)?;
        } else {
            terminal.clear()?;
        }
        Ok(terminal)
    }

    fn restore_terminal(&mut self) -> Result<()> {
        ratatui::restore();
        if !self.config.is_inline() {
            execute!(io::stdout(), LeaveAlternateScreen)?
        } else {
            self.terminal.clear()?;
        }
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal
            .draw(|frame| frame.render_widget(&mut self.tui, self.config.area(frame.area())))?;
        Ok(())
    }

    fn draw_forced(&mut self) -> Result<()> {
        self.draw()?;
        self.state.skip_frame();
        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Exit => self.state.exit(),
            Action::Draw => self.draw_forced()?,
            Action::Tui(action) => self.tui.handle_action(action),
        }
        Ok(())
    }

    fn handle_key(&mut self, key: Key) -> Result<()> {
        if let Some(action) = self.config.action_for_key(&key) {
            self.handle_action(action)?;
        }
        Ok(())
    }
}
