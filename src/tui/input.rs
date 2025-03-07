use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};
use tui_textarea::{CursorMove, TextArea};

use crate::types::action::InputAction;

pub struct Input<'a> {
    text_area: TextArea<'a>,
}

impl Input<'_> {
    pub fn new() -> Self {
        let mut text_area = TextArea::from([""]);
        text_area.set_block(Block::bordered());
        Input { text_area }
    }

    #[rustfmt::skip]
    pub fn handle_action(&mut self, action: InputAction) -> Option<&str> {
        if match action {
            InputAction::Key(key) => self.text_area.input_without_shortcuts(key),
            InputAction::InsertNewline => { self.text_area.insert_newline(); true },
            // Some move actions return true because they can change the selected line
            InputAction::MoveForward => { self.text_area.move_cursor(CursorMove::Forward); true },
            InputAction::MoveBack => { self.text_area.move_cursor(CursorMove::Back); true },
            InputAction::MoveUp => { self.text_area.move_cursor(CursorMove::Up); true },
            InputAction::MoveDown => { self.text_area.move_cursor(CursorMove::Down); true },
            InputAction::MoveForwardWord => { self.text_area.move_cursor(CursorMove::WordForward); true },
            InputAction::MoveBackWord => { self.text_area.move_cursor(CursorMove::WordBack); true },
            InputAction::MoveToEndOfWord => { self.text_area.move_cursor(CursorMove::WordEnd); true },
            InputAction::MoveToTop => { self.text_area.move_cursor(CursorMove::Top); true },
            InputAction::MoveToBottom => { self.text_area.move_cursor(CursorMove::Bottom); true },
            InputAction::MoveToHead => { self.text_area.move_cursor(CursorMove::Head); false },
            InputAction::MoveToEnd => { self.text_area.move_cursor(CursorMove::End); false },
            InputAction::Delete => self.text_area.delete_char(),
            InputAction::DeleteNext => self.text_area.delete_next_char(),
            InputAction::DeleteWord => self.text_area.delete_word(),
            InputAction::DeleteNextWord => self.text_area.delete_next_word(),
            InputAction::DeleteToHead => self.text_area.delete_line_by_head(),
            InputAction::DeleteToEnd => self.text_area.delete_line_by_end(),
        } {
            self.text_area.lines().get(self.text_area.cursor().0).map(|s| s.as_str())
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
