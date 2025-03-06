use derive_more::From;
use futures::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

use crossterm::event::{Event, EventStream};

use crate::types::{action::Action, key::Key};

#[derive(From)]
pub enum Message {
    Error(anyhow::Error),
    Action(Action),
    Key(Key),
}

pub async fn handle_events(sender: UnboundedSender<Message>) {
    while let Some(event) = EventStream::new().next().await {
        if let Some(message) = match event {
            Err(error) => Some(Message::Error(error.into())),
            Ok(Event::Key(key)) => Some(Message::Key(key.into())),
            _ => None,
        } {
            let _ = sender.send(message);
        }
    }
}
