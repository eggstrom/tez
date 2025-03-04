use derive_more::From;
use futures::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};

use crate::types::action::{Action, TuiAction};

#[derive(From)]
pub enum Message {
    Error(anyhow::Error),
    Action(Action),
}

pub async fn handle_events(sender: UnboundedSender<Message>) {
    let mut stream = EventStream::new();
    while let Some(event) = stream.next().await {
        if let Some(action) = match event.map(handle_event) {
            Ok(Some(action)) => Some(action.into()),
            Ok(None) => None,
            Err(error) => Some(Message::Error(error.into())),
        } {
            let _ = sender.send(action);
        }
    }
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
