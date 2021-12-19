use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod events;
pub mod opts;
pub mod ui;

fn main() {
    match opts::Flags::get() {
        opts::Flags::Help => {
            show_help();
        }
        opts::Flags::Filename(fname) => run(fname),
    };
}

fn show_help() {
    println!("HALP!");
}

fn run(filename: String) {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Unable to bootstrap terminal");
    let mut app: app::State = app::State::new(&filename);

    enable_raw_mode().expect("Could not enable raw mode");
    loop {
        terminal
            .draw(|frame| ui::draw(frame, &mut app))
            .expect("Could not draw UI");
        let sig = events::handler(&mut app);
        match sig {
            app::Signal::Quit => {
                disable_raw_mode().expect("Could not disable raw mode");
                return;
            }
            _ => continue,
        }
    }
}
