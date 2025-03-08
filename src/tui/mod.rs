use anyhow::Result;
use input::Input;
use list::SearchableList;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};
use tokio::sync::watch::Sender;

use crate::{searcher::SearcherSource, types::action::TuiAction};

mod input;
mod lazy;
mod list;

pub struct Tui<'a> {
    input: Input<'a>,
    list: SearchableList<'a>,
}

impl Tui<'_> {
    pub fn new(draw_sender: Sender<()>) -> Result<Self> {
        let input = Input::new();
        let list = SearchableList::new(SearcherSource::Stdin, draw_sender);
        Ok(Tui { input, list })
    }

    pub fn handle_action(&mut self, action: TuiAction) {
        match action {
            TuiAction::Next => self.list.next(),
            TuiAction::Previous => self.list.previous(),
            TuiAction::First => self.list.first(),
            TuiAction::Last => self.list.last(),
            TuiAction::Input(action) => {
                if let Some(text) = self.input.handle_action(action) {
                    self.list.search(text);
                }
            }
        }
    }
}

impl Widget for &mut Tui<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [top_area, bottom_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(5)]).areas(area);
        self.list.render(top_area, buf);
        self.input.render(bottom_area, buf);
    }
}
