mod status;

use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};
use status::Status;

use crate::system::System;

pub struct App;

impl StatefulWidget for App {
    type State = System;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title_top("System Monitor");
        let inner = block.inner(area);
        block.render(area, buf);
        let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let layout = layout.split(inner);
        Status(state).render(layout[0], buf);
    }
}
