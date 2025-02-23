use std::io;

use anyhow::Result;
use list::PlainList;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::actions::TuiAction;

mod list;

pub struct Tui<'a> {
    list: PlainList<'a>,
}

impl Tui<'_> {
    pub fn new() -> Result<Self> {
        let items = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
        Ok(Tui {
            list: PlainList::new(items.into_iter()),
        })
    }

    pub fn handle_action(&mut self, action: TuiAction) {
        match action {
            TuiAction::ScrollDown => self.list.next(),
            TuiAction::ScrollUp => self.list.prev(),
        }
    }
}

impl Widget for &mut Tui<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.list.render(area, buf);
    }
}
