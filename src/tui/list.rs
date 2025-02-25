use std::sync::Arc;

use nucleo::{
    pattern::{CaseMatching, Normalization},
    Nucleo, Status,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{
        Block, List, ListItem, ListState, Scrollbar, ScrollbarState, StatefulWidget, Widget,
    },
};

pub struct PlainList<'a> {
    list: List<'a>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
}
impl<'a> PlainList<'a> {
    pub fn new<I>(items: I) -> Self
    where
        I: ExactSizeIterator,
        I::Item: Into<ListItem<'a>>,
    {
        let len = items.len();
        let list = List::new(items)
            .highlight_style(Color::Red)
            .block(Block::bordered());
        let list_state = ListState::default().with_selected(Some(0));
        let scrollbar_state = ScrollbarState::new(len).viewport_content_length(1);
        PlainList {
            list,
            list_state,
            scrollbar_state,
        }
    }

    pub fn next(&mut self) {
        self.list_state.select_next();
        self.scrollbar_state.next();
    }

    pub fn prev(&mut self) {
        self.list_state.select_previous();
        self.scrollbar_state.prev();
    }
}

impl Widget for &mut PlainList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let scrollbar = Scrollbar::default();

        StatefulWidget::render(&self.list, area, buf, &mut self.list_state);
        scrollbar.render(area, buf, &mut self.scrollbar_state);
    }
}

pub struct SearchableList<'a> {
    searcher: Searcher,
    list: List<'a>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
}

impl<'a> SearchableList<'a> {
    pub fn new<I>(items: I) -> Self
    where
        I: ExactSizeIterator<Item = String>,
    {
        let searcher = Searcher::new(items);
        let list = Self::new_list(searcher.matches());
        let list_state = ListState::default().with_selected(Some(0));
        SearchableList {
            searcher,
            list,
            list_state,
            scrollbar_state: ScrollbarState::default(),
        }
    }

    fn new_list(items: Vec<String>) -> List<'a> {
        List::new(items)
            .highlight_style(Color::Red)
            .block(Block::bordered())
    }

    pub fn next(&mut self) {
        self.list_state.select_next();
        self.scrollbar_state.next();
    }

    pub fn prev(&mut self) {
        self.list_state.select_previous();
        self.scrollbar_state.prev();
    }

    pub fn search(&mut self, s: &str) {
        self.searcher.search(s);
        self.list = Self::new_list(self.searcher.matches());
    }
}

impl Widget for &mut SearchableList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let scrollbar = Scrollbar::default();

        StatefulWidget::render(&self.list, area, buf, &mut self.list_state);
        scrollbar.render(area, buf, &mut self.scrollbar_state);
    }
}

struct Searcher {
    nucleo: Nucleo<String>,
    last_pattern: String,
}

impl Searcher {
    pub fn new<I>(items: I) -> Self
    where
        I: ExactSizeIterator<Item = String>,
    {
        let config = nucleo::Config::DEFAULT;
        let nucleo = Nucleo::new(config, Arc::new(|| {}), None, 1);
        let injector = nucleo.injector();
        items.for_each(|item| {
            injector.push(item, |item, columns| {
                columns[0] = item.as_str().into();
            });
        });

        let mut searcher = Searcher {
            nucleo,
            last_pattern: String::new(),
        };
        searcher.search("");
        searcher
    }

    pub fn matches(&self) -> Vec<String> {
        self.nucleo
            .snapshot()
            .matched_items(..)
            .map(|item| item.data.clone())
            .collect()
    }

    pub fn search(&mut self, pattern: &str) {
        self.nucleo.pattern.reparse(
            0,
            pattern,
            CaseMatching::Smart,
            Normalization::Smart,
            pattern == self.last_pattern,
        );
        while let Status { running: true, .. } = self.nucleo.tick(0) {}
    }
}
