use ratatui::{
    style::{Style, Stylize},
    widgets::{Gauge, Widget},
};

use crate::system::System;

pub struct Status<'a>(pub &'a System);

impl Widget for Status<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let gauge = Gauge::default()
            .gauge_style(Style::new().green().on_black().italic())
            .percent(self.0.battery_percentage as u16);
        gauge.render(area, buf);
    }
}
