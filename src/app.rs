use std::{fs, io::{self, BufRead}};
use std::process::Command;

use super::ui;
use super::ui::Pane;

pub enum Signal {
    Quit,
    Nop,
    Run,
    Mode,
}

pub enum Mode {
    Internal,
    Shell,
}

pub struct State {
    pub output: ui::panels::Content,
    pub source: ui::panels::Content,
    pub filename: String,

    command: ui::panels::Command,
    internal: ui::panels::Command,
    active: ui::Panel,
    mode: Mode,
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
        let command = ui::panels::Command::new(String::from(".|keys"));
        let output = run_command(&command.get_content(), filename);

        State {
            filename: String::from(filename),
            source: ui::panels::Content::new(String::from(source), ui::Panel::Source),
            output: ui::panels::Content::new(output, ui::Panel::Output),

            command,
            internal: ui::panels::Command::new(String::from("")),
            active: ui::Panel::Command,
            mode: Mode::Shell,
        }
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn command(&self) -> &ui::panels::Command {
        match self.mode {
            Mode::Shell => &self.command,
            Mode::Internal => &self.internal,
        }
    }

    pub fn get_mut_active(&mut self) -> Box<&mut dyn ui::Pane> {
        if ui::Panel::Source == self.active {
            return Box::new(&mut self.source);
        }
        if ui::Panel::Output == self.active {
            return Box::new(&mut self.output);
        }
        Box::new(&mut self.command)
    }

    pub fn get_active(&self) -> Box<&dyn ui::Pane> {
        if ui::Panel::Source == self.active {
            return Box::new(&self.source);
        }
        if ui::Panel::Output == self.active {
            return Box::new(&self.output);
        }
        Box::new(&self.command)
    }

    pub fn set_active(&mut self, active: ui::Panel) {
        self.active = active;
    }

    pub fn run_current_command(&mut self) {
        self.command.record();
        let output = run_command(&self.command.get_content(), self.filename.as_str());
        self.output = ui::panels::Content::new(output, ui::Panel::Output);
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
