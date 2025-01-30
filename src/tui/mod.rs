mod app;
mod components;

use std::time::Duration;

use app::App;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

use crate::system::System;

pub fn start() -> anyhow::Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> anyhow::Result<()> {
    let mut system = System::monitor()?;
    loop {
        terminal.draw(|frame| render(frame, &mut system))?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, system: &mut System) {
    let app = App;
    frame.render_stateful_widget(app, frame.area(), system);
}
