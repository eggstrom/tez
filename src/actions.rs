use std::{sync::mpsc::Sender, thread};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use derive_more::From;

#[derive(From)]
pub enum Action {
    Error(anyhow::Error),
    Quit,
    #[from]
    Tui(TuiAction),
}

pub enum TuiAction {
    ScrollDown,
    ScrollUp,
}

pub fn handle_events(sender: Sender<Action>) {
    thread::spawn(move || loop {
        if let Some(action) = match event::read().map(|event| handle_event(event)) {
            Ok(Some(action)) => Some(action),
            Err(error) => Some(Action::Error(error.into())),
            _ => None,
        } {
            let _ = sender.send(action);
        }
    });
}

fn handle_event(event: Event) -> Option<Action> {
    match event {
        Event::Key(event) => handle_key_event(event),
        _ => None,
    }
}

fn handle_key_event(event: KeyEvent) -> Option<Action> {
    match event {
        KeyEvent {
            code,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => match code {
            KeyCode::Char('c') => Some(Action::Quit),
            KeyCode::Char('n') => Some(TuiAction::ScrollDown.into()),
            KeyCode::Char('p') => Some(TuiAction::ScrollUp.into()),
            _ => None,
        },
        _ => None,
    }
}
