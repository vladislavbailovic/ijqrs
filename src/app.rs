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
    pub output: String,
    pub source: String,
    pub active_panel: ui::Panel,
    pub source_pos: usize,
    pub output_pos: usize
}

impl State {
    pub fn new(filename: &str) -> State {
        let source = fs::read_to_string(filename)
            .expect("Error reading file");
        let command = String::from(".|keys");
        let output = run_command(&command);
        return State{
            filename: String::from(filename),
            active_panel: ui::Panel::Output,
            source_pos: 0, output_pos: 0,
            command, output, source };
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
