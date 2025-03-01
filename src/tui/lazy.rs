use std::mem;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, ListState, StatefulWidget},
};

#[derive(Default, Clone, Copy)]
enum LazyPos {
    #[default]
    None,
    // It's not guaranteed the entire LazyList'll be loaded when the user wraps
    // around to the other side, meaning they'll be stuck somewhere in the
    // middle if they're too quick. Start and End help remember the direction of
    // the last wrap.
    Start(usize),
    End(usize),
}

#[derive(Default)]
struct LazyState {
    len: usize,
    pos: LazyPos,
    offset: usize,
}

impl LazyState {
    pub fn set_len(&mut self, len: usize) {
        self.len = len;
        match self.pos {
            LazyPos::Start(pos) if pos >= len => self.pos = LazyPos::Start(len - 1),
            LazyPos::End(pos) if pos >= len => self.pos = LazyPos::End(len - 1),
            _ => (),
        }
    }

    pub fn next(&mut self) {
        self.pos = match (self.len, self.pos) {
            (0, _) => LazyPos::None,
            (_, LazyPos::Start(pos)) if pos < self.len - 1 => LazyPos::Start(pos + 1),
            (_, LazyPos::End(pos)) if pos > 0 => LazyPos::End(pos - 1),
            _ => LazyPos::Start(0),
        };
    }

    pub fn previous(&mut self) {
        self.pos = match (self.len, self.pos) {
            (0, _) => LazyPos::None,
            (_, LazyPos::End(pos)) if pos < self.len - 1 => LazyPos::End(pos + 1),
            (_, LazyPos::Start(pos)) if pos > 0 => LazyPos::Start(pos - 1),
            _ => LazyPos::End(0),
        };
    }

    pub fn update_offset(&mut self, height: u16) {
        if let Some(selected) = self.position() {
            self.offset = self
                .offset
                .clamp(selected.saturating_sub(height as usize - 1), selected);
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn position(&self) -> Option<usize> {
        match self.pos {
            LazyPos::None => None,
            LazyPos::Start(pos) => Some(pos),
            LazyPos::End(pos) => Some(self.len - 1 - pos),
        }
    }

    pub fn real_position(&self) -> Option<usize> {
        self.position().map(|pos| pos - self.offset())
    }
}

pub struct LazyList<'a> {
    builder: fn() -> List<'a>,
    state: LazyState,
}

impl<'a> LazyList<'a> {
    pub fn new(builder: fn() -> List<'a>) -> Self {
        LazyList {
            builder,
            state: LazyState::default(),
        }
    }

    pub fn next(&mut self) {
        self.state.next();
    }

    pub fn previous(&mut self) {
        self.state.previous();
    }

    pub fn update_height(&mut self, height: u16) {
        self.state.update_offset(height);
    }

    pub fn offset(&self) -> usize {
        self.state.offset()
    }
}

pub struct LazyListState {
    len: usize,
    results: Vec<String>,
}

impl LazyListState {
    pub fn new(len: usize, results: Vec<String>) -> Self {
        LazyListState { len, results }
    }
}

impl StatefulWidget for &mut LazyList<'_> {
    type State = LazyListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.state.set_len(state.len);

        let list = (self.builder)().items(mem::take(&mut state.results));
        let mut list_state = ListState::default().with_selected(self.state.real_position());
        StatefulWidget::render(&list, area, buf, &mut list_state);
    }
}
