use std::{sync::Arc, time::Duration};

use nucleo::{
    pattern::{CaseMatching, Normalization},
    Injector, Nucleo,
};
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    sync::{
        mpsc::UnboundedSender,
        watch::{Receiver, Sender},
    },
    task,
    time::sleep,
};

use crate::types::action::Action;

pub enum SearcherSource {
    Stdin,
    Command(String),
}

impl SearcherSource {
    pub fn inject(&self, injector: Injector<String>) {
        task::spawn(match self {
            SearcherSource::Stdin => SearcherSource::inject_stdin(injector),
            SearcherSource::Command(_) => todo!(),
        });
    }

    async fn inject_stdin(injector: Injector<String>) {
        let mut lines = BufReader::new(io::stdin()).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            injector.push(line, |line, columns| {
                columns[0] = line.as_str().into();
            });
        }
    }
}

pub struct Searcher {
    nucleo: Nucleo<String>,
    source: Arc<SearcherSource>,
    last_pattern: Option<String>,
}

impl Searcher {
    pub fn new(source: SearcherSource, draw_sender: Sender<()>) -> Self {
        let nucleo = Nucleo::new(
            nucleo::Config::DEFAULT,
            Arc::new(move || {
                let _ = draw_sender.send(());
            }),
            None,
            1,
        );

        let mut searcher = Searcher {
            nucleo,
            source: Arc::new(source),
            last_pattern: None,
        };
        searcher.search("");
        searcher.init();
        searcher
    }

    pub fn init(&mut self) {
        let source = Arc::clone(&self.source);
        let injector = self.nucleo.injector();
        source.inject(injector);
    }

    pub fn search(&mut self, pattern: &str) {
        if self
            .last_pattern
            .as_ref()
            .is_some_and(|last_pattern| pattern == last_pattern)
        {
            return;
        }

        self.nucleo.pattern.reparse(
            0,
            pattern,
            CaseMatching::Smart,
            Normalization::Smart,
            self.last_pattern
                .as_ref()
                .is_some_and(|last_pattern| pattern.starts_with(last_pattern)),
        );
        self.last_pattern = Some(pattern.to_string());
    }

    pub fn tick(&mut self) {
        self.nucleo.tick(10);
    }

    pub fn result_count(&self) -> usize {
        self.nucleo.snapshot().matched_item_count() as usize
    }

    pub fn results(&self, offset: usize, height: u16) -> Vec<String> {
        let (offset, height) = (offset as u32, height as u32);
        let snapshot = self.nucleo.snapshot();
        let max = snapshot.matched_item_count();
        snapshot
            .matched_items(offset.min(max)..(offset + height).min(max))
            .map(|item| item.data.clone())
            .collect()
    }
}

pub async fn debounce_draws(mut draw_receiver: Receiver<()>, sender: UnboundedSender<Action>) {
    while draw_receiver.changed().await.is_ok() {
        let _ = sender.send(Action::Draw);
        sleep(Duration::from_millis(100)).await;
    }
}
