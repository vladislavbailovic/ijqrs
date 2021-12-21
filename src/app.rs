use std::{fs, io::{self, BufRead}};
use std::process::Command;

use super::ui;
use super::ui::Pane;

pub enum Signal {
    Quit,
    Nop,
}

pub struct State {
    pub command: ui::CommandPanel,
    pub output: ui::ContentPanel,
    pub source: ui::ContentPanel,

    pub active_panel: ui::Panel,
    pub filename: String,
}

impl State {
    pub fn from_file(filename: &str) -> State {
        let source = fs::read_to_string(filename).expect("Error reading file");
        State::new(filename, &source)
    }
    pub fn from_stdin() -> State {
        let mut source = String::from("");
        for line in io::stdin().lock().lines() {
            source += (String::from(line.expect("IO Error")) + "\n").as_str();
        }
        let filename = write_temp(&source);
        State::new(&filename, &source)
    }

    fn new(filename: &str, source: &str) -> State {
        let command = ui::CommandPanel::new(String::from(".|keys"));
        let output = run_command(&command.get_content(), filename);

        State {
            filename: String::from(filename),
            active_panel: ui::Panel::Command,
            command,
            source: ui::ContentPanel::new(String::from(source)),
            output: ui::ContentPanel::new(output),
        }
    }

    pub fn run_current_command(&mut self) {
        self.command.record();
        let output = run_command(&self.command.get_content(), self.filename.as_str());
        self.output = ui::ContentPanel::new(output);
    }

    pub fn prev_from_history(&mut self) {
        self.command.prev_from_history();
    }

    pub fn next_from_history(&mut self) {
        self.command.next_from_history();
    }

    pub fn scroll_down(&mut self) {
        match self.active_panel {
            ui::Panel::Source => self.source.scroll_down(),
            ui::Panel::Output => self.output.scroll_down(),
            _ => {}
        };
    }

    pub fn scroll_up(&mut self) {
        match self.active_panel {
            ui::Panel::Source => {
                self.source.scroll_up();
            }
            ui::Panel::Output => {
                self.output.scroll_up();
            }
            _ => {}
        };
    }

    pub fn scroll_pos(&self, panel: ui::Panel) -> (u16, u16) {
        let x = 0;
        let y = match panel {
            ui::Panel::Source => self.source.get_pos(),
            ui::Panel::Output => self.output.get_pos(),
            _ => 0,
        };
        (y as u16, x)
    }
}

fn run_command(command: &str, filename: &str) -> String {
    let command = Command::new("jq")
        .arg(command)
        .arg(filename)
        .output()
        .expect("Command execution failed");
    let result = String::from_utf8(command.stdout).expect("Invalid stdout");
    if result.is_empty() {
        return String::from_utf8(command.stderr).expect("Invalid stderr");
    }
    result
}

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::Write;

fn write_temp(source: &str) -> String {
    let mut tmp = env::temp_dir();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error figuring out current time");
    let fname = format!("ijqrs-{}.json", now.as_nanos());
    tmp.push(fname);
    let path = tmp.to_str().expect("Error getting temporary file path");

    let mut file = File::create(path).expect("Error creating temp file");
    file.write_all(source.as_bytes());
    return String::from(path);
}
