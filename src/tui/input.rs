use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};
use tui_textarea::TextArea;

pub struct Input<'a> {
    text_area: TextArea<'a>,
}

impl Input<'_> {
    pub fn new() -> Self {
        let mut text_area = TextArea::from([""]);
        text_area.set_block(Block::bordered());
        Input { text_area }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> Option<&str> {
        if self.text_area.input(event) {
            self.text_area.lines().first().map(|s| s.as_str())
        } else {
            None
        }
    }
}

impl Widget for &Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.text_area.render(area, buf);
    }
}
