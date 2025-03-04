use std::{sync::mpsc::Sender, thread};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::types::action::{Action, TuiAction};

pub fn handle_events(sender: Sender<Action>) {
    thread::spawn(move || loop {
        if let Some(action) = match event::read().map(handle_event) {
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
    let action = match event {
        KeyEvent {
            code,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => match code {
            KeyCode::Char('c') => Some(Action::Exit),
            KeyCode::Char('n') => Some(TuiAction::Next.into()),
            KeyCode::Char('p') => Some(TuiAction::Previous.into()),
            _ => None,
        },
        KeyEvent {
            code,
            modifiers: KeyModifiers::ALT,
            ..
        } => match code {
            KeyCode::Char('a') => Some(TuiAction::First.into()),
            KeyCode::Char('e') => Some(TuiAction::Last.into()),
            _ => None,
        },
        _ => None,
    };

    // This is temporary
    match action {
        Some(_) => action,
        None => Some(TuiAction::Key(event).into()),
    }
}
