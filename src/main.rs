use std::io;

use tui::{
    Terminal,
    backend::CrosstermBackend
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode}
};

pub mod events;
pub mod ui;
pub mod app;

fn main() {
    let filename = "./examples/generated.json";

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Unable to bootstrap terminal");
    let mut app: app::State = app::State::new(&filename);

    enable_raw_mode();
    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app));
        let sig = events::handler(&mut app);
        match sig {
            app::Signal::Quit => { disable_raw_mode(); return; },
            _ => continue,
        }
    };
}
