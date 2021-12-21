use std::{fs, io::{self, BufRead}};
use std::process::Command;

use super::ui;

pub enum Signal {
    Quit,
    Nop,
}

pub struct State {
    pub output: String,
    pub command: String,
    pub source: String,
    pub active_panel: ui::Panel,
    pub filename: String,

    command_history: Vec<String>,

    source_pos: ui::Scroller,
    output_pos: ui::Scroller,
    history_pos: ui::Scroller,
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
        let command = String::from(".|keys");

        let srclines: Vec<&str> = source.split("\n").collect();

        let output = run_command(&command, filename);
        let outlines: Vec<&str> = output.split("\n").collect();

        State {
            filename: String::from(filename),
            command_history: vec![command.as_str().to_string()],
            active_panel: ui::Panel::Command,
            source: String::from(source),
            command,
            output: String::from(output.as_str()),

            source_pos: ui::Scroller::new(srclines.len()),
            output_pos: ui::Scroller::new(outlines.len()),
            history_pos: ui::Scroller::new(0),
        }
    }

    pub fn run_current_command(&mut self) {
        self.command_history.push(self.command.to_string());
        self.history_pos.set_max(self.command_history.len() - 1);

        let output = run_command(&self.command, self.filename.as_str());
        let outlines: Vec<&str> = output.split("\n").collect();
        self.output = String::from(output.as_str());
        self.output_pos.set_max(outlines.len());
    }

    pub fn prev_from_history(&mut self) {
        self.history_pos.prev();
        self.command = self.command_history[self.history_pos.get()].as_str().to_string();
    }

    pub fn next_from_history(&mut self) {
        self.history_pos.next();
        self.command = self.command_history[self.history_pos.get()].as_str().to_string();
    }

    pub fn scroll_down(&mut self) {
        match self.active_panel {
            ui::Panel::Source => self.source_pos.next(),
            ui::Panel::Output => self.output_pos.next(),
            _ => {}
        };
    }

    pub fn scroll_up(&mut self) {
        match self.active_panel {
            ui::Panel::Source => {
                self.source_pos.prev();
            }
            ui::Panel::Output => {
                self.output_pos.prev();
            }
            _ => {}
        };
    }

    pub fn scroll_pos(&self, panel: ui::Panel) -> (u16, u16) {
        let x = 0;
        let y = match panel {
            ui::Panel::Source => self.source_pos.get(),
            ui::Panel::Output => self.output_pos.get(),
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
