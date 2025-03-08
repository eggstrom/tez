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
    pub fn update(&mut self, len: usize, height: u16) {
        self.update_len(len);
        self.update_height(height);
    }

    fn update_len(&mut self, len: usize) {
        self.len = len;
        match self.pos {
            LazyPos::Start(pos) if pos >= len => self.pos = LazyPos::Start(len.saturating_sub(1)),
            LazyPos::End(pos) if pos >= len => self.pos = LazyPos::End(len.saturating_sub(1)),
            _ => (),
        }
    }

    fn update_height(&mut self, height: u16) {
        if let Some(pos) = self.position() {
            self.offset = self
                .offset
                .clamp(pos.saturating_sub((height as usize).saturating_sub(1)), pos)
                .min(self.len.saturating_sub(height as usize));
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

    pub fn first(&mut self) {
        self.pos = LazyPos::Start(0);
    }

    pub fn last(&mut self) {
        self.pos = LazyPos::End(0);
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn position(&self) -> Option<usize> {
        match self.pos {
            LazyPos::None => None,
            LazyPos::Start(pos) => Some(pos),
            LazyPos::End(pos) => Some(self.len.saturating_sub(pos + 1)),
        }
    }

    pub fn real_position(&self) -> Option<usize> {
        self.position().map(|pos| pos.saturating_sub(self.offset()))
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

    pub fn first(&mut self) {
        self.state.first();
    }

    pub fn last(&mut self) {
        self.state.last();
    }

    pub fn update(&mut self, len: usize, height: u16) {
        self.state.update(len, height);
    }

    pub fn offset(&self) -> usize {
        self.state.offset()
    }
}

impl StatefulWidget for &mut LazyList<'_> {
    type State = Vec<String>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list = (self.builder)().items(mem::take(state));
        let mut list_state = ListState::default().with_selected(self.state.real_position());
        StatefulWidget::render(&list, area, buf, &mut list_state);
    }
}
