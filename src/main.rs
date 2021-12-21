use std::io;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod events;
pub mod opts;
pub mod ui;

use ui::Pane;

fn main() {
    match opts::Flags::get() {
        opts::Flags::Help => {
            show_help();
        },
        opts::Flags::Stdin => {
            let mut app: app::State = app::State::from_stdin();
            run(&mut app);
        },
        opts::Flags::Filename(fname) => {
            let mut app: app::State = app::State::from_file(&fname);
            run(&mut app);
        }
    };
}

// TODO: Show actual help and usage
fn show_help() {
    println!("HALP!");
}

fn run(app: &mut app::State) {
    execute!(io::stdout(), EnterAlternateScreen).expect("Unable to enter alternate screen");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Unable to bootstrap terminal");

    enable_raw_mode().expect("Could not enable raw mode");
    loop {
        terminal
            .draw(|frame| ui::draw(frame, app))
            .expect("Could not draw UI");
        let sig = events::handler(app);
        match sig {
            app::Signal::Quit => {
                disable_raw_mode().expect("Could not disable raw mode");
                execute!(io::stdout(), LeaveAlternateScreen)
                    .expect("Unable to leave alternate screen");
                println!("jq '{}' {}", app.command.get_content(), app.filename);
                return;
            }
            _ => continue,
        }
    }
}
