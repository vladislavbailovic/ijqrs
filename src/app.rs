use std::process::Command;

use super::ui;

pub enum Signal {
    Quit,
    Nop,
}

pub struct State {
    pub command: String,
    pub output: String,
    pub source: String,
    pub active_panel: ui::Panel,
    pub source_pos: usize,
    pub output_pos: usize
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
