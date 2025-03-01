use std::sync::mpsc::Sender;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{
        Block, List, ListItem, ListState, Scrollbar, ScrollbarState, StatefulWidget, Widget,
    },
};

use crate::{
    actions::Action,
    searcher::{Searcher, SearcherSource},
};

use super::lazy::{LazyList, LazyListState};

pub struct PlainList<'a> {
    list: List<'a>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
}
impl<'a> PlainList<'a> {
    pub fn new<I>(items: I) -> Self
    where
        I: Iterator,
        I::Item: Into<ListItem<'a>>,
    {
        let list = List::new(items)
            .highlight_style(Color::Red)
            .block(Block::bordered());
        let list_state = ListState::default().with_selected(Some(0));
        let scrollbar_state = ScrollbarState::new(list.len()).viewport_content_length(1);
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

    pub fn previous(&mut self) {
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
    list: LazyList<'a>,
    scrollbar_state: ScrollbarState,
}

impl SearchableList<'_> {
    pub fn new(sender: Sender<Action>, source: SearcherSource) -> Self {
        let list_builder = || {
            List::default()
                .highlight_style(Color::Red)
                .block(Block::bordered())
        };
        SearchableList {
            searcher: Searcher::new(sender, source),
            list: LazyList::new(list_builder),
            scrollbar_state: ScrollbarState::default(),
        }
    }

    pub fn first(&mut self) {
        self.list.first();
    }

    pub fn last(&mut self) {
        self.list.last();
    }

    pub fn next(&mut self) {
        self.list.next();
        self.scrollbar_state.next();
    }

    pub fn previous(&mut self) {
        self.list.previous();
        self.scrollbar_state.prev();
    }

    pub fn search(&mut self, s: &str) {
        self.searcher.search(s);
    }
}

impl Widget for &mut SearchableList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.searcher.tick();

        let height = area.height - 2;
        self.list.update_height(height);
        let len = self.searcher.result_count();
        let results = self.searcher.results(self.list.offset(), height);
        self.list
            .render(area, buf, &mut LazyListState::new(len, results));

        let scrollbar = Scrollbar::default();
        scrollbar.render(area, buf, &mut self.scrollbar_state);
    }
}
