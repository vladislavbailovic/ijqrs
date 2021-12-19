use std::{fs, io};

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
    let source = fs::read_to_string(filename)
        .expect("Error reading file");
    let command = String::from(".|keys");
    let output = app::run_command(&command);
    let app: &mut app::State = &mut app::State{
        active_panel: ui::Panel::Output,
        source_pos: 0, output_pos: 0,
        command, output, source };

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Unable to bootstrap terminal");

    enable_raw_mode();
    loop {
        terminal.draw(|frame| ui::draw(frame, app));
        let sig = events::handler(app);
        match sig {
            app::Signal::Quit => { disable_raw_mode(); return; },
            _ => continue,
        }
    };
}
