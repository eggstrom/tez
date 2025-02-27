use std::{mem, sync::mpsc::Sender};

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
    searcher::{SearchResults, Searcher, SearcherSource},
};

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

pub struct LazyList<'a> {
    builder: fn() -> List<'a>,
    len: Option<usize>,
    offset: usize,
    selected: Option<usize>,
}

impl<'a> LazyList<'a> {
    pub fn new(builder: fn() -> List<'a>) -> Self {
        LazyList {
            builder,
            len: None,
            offset: 0,
            selected: None,
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    pub fn select_next(&mut self) {
        let len = match self.len {
            None => return,
            Some(len) => len,
        };
    }

    pub fn select_previous(&mut self) {
        let len = match self.len {
            None => return,
            Some(len) => len,
        };
    }
}

impl StatefulWidget for &mut LazyList<'_> {
    type State = SearchResults;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut SearchResults) {
        self.len = Some(state.len);
        let list = (self.builder)().items(mem::take(&mut state.items));

        let mut list_state = ListState::default()
            .with_offset(0)
            .with_selected(self.selected.map(|sel| sel - self.offset));
        StatefulWidget::render(&list, area, buf, &mut list_state);
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

    pub fn next(&mut self) {
        self.list.select_next();
        self.scrollbar_state.next();
    }

    pub fn previous(&mut self) {
        self.list.select_previous();
        self.scrollbar_state.prev();
    }

    pub fn search(&mut self, s: &str) {
        self.searcher.search(s);
    }
}

impl Widget for &mut SearchableList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.searcher.tick();

        let mut results = self.searcher.results(self.list.offset, area.height);
        self.list.render(area, buf, &mut results);

        let scrollbar = Scrollbar::default();
        scrollbar.render(area, buf, &mut self.scrollbar_state);
    }
}
