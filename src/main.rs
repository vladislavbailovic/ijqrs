use std::io;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

pub mod actions;
pub mod app;
pub mod events;
pub mod help;
pub mod opts;
pub mod ui;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    match opts::Flags::get() {
        opts::Flags::Help => {
            show_help();
        }
        opts::Flags::Version => {
            show_version();
        }
        opts::Flags::Stdin => {
            let mut app: app::State = app::State::from_stdin();
            run(&mut app);
        }
        opts::Flags::Filename(fname) => {
            let mut app: app::State = app::State::from_file(&fname);
            run(&mut app);
        }
    };
}

fn show_help() {
    println!("{}", help::usage());
}

fn show_version() {
    println!("{}", VERSION);
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
                terminal.clear().expect("Unable to clear the terminal");
                terminal
                    .show_cursor()
                    .expect("Unable to re-show the cursor");
                disable_raw_mode().expect("Could not disable raw mode");
                execute!(io::stdout(), LeaveAlternateScreen)
                    .expect("Unable to leave alternate screen");
                return;
            },
            app::Signal::Run => {
                app.run_current_command();
            },
            app::Signal::Bookmark => {
                app.add_bookmark();
            },
            app::Signal::LoadBookmark(bm) => {
                app.load_bookmark(bm);
            },
            app::Signal::Nop => continue,
        }
    }
}
