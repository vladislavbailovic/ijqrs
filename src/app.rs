use std::process::Command;
use std::fs;

use super::ui;

pub enum Signal {
    Quit,
    Nop,
}

pub struct State {
    pub filename: String,
    pub command: String,
    pub command_history: Vec<String>,
    pub output: String,
    pub source: String,
    pub active_panel: ui::Panel,
    pub source_pos: usize,
    pub output_pos: usize,

    history_pos: usize
}

impl State {
    pub fn new(filename: &str) -> State {
        let source = fs::read_to_string(filename)
            .expect("Error reading file");
        let command = String::from(".|keys");
        let output = run_command(&command);
        State{
            filename: String::from(filename),
            command_history: vec![command.as_str().to_string()],
            active_panel: ui::Panel::Command,
            source_pos: 0, output_pos: 0,
            command, output, source,

            history_pos: 0
        }
    }

    pub fn run_current_command(&mut self) {
        self.command_history.push(self.command.to_string());
        self.history_pos = self.command_history.len() - 1;
        self.output = run_command(&self.command);
    }

    pub fn prev_from_history(&mut self) {
        if self.history_pos > 0 {
            self.history_pos -= 1;
            self.command = self.command_history[self.history_pos].as_str().to_string();
        }
    }

    pub fn next_from_history(&mut self) {
        if self.history_pos < self.command_history.len() - 1 {
            self.history_pos += 1;
            self.command = self.command_history[self.history_pos].as_str().to_string();
        }
    }
}

pub fn run_command(command: &str) -> String {
    let command = Command::new("jq")
            .arg(command)
            .arg("./examples/generated.json")
            .output().expect("Command execution failed");
    let result = String::from_utf8(command.stdout).expect("Invalid stdout");
    if "" == result {
        return String::from_utf8(command.stderr).expect("Invalid stdout");
    }
    result
}
