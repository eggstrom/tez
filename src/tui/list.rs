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
