use std::fs;
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

    filename: String,
    command_history: Vec<String>,

    source_pos: usize,
    output_pos: usize,
    history_pos: usize,
}

impl State {
    pub fn new(filename: &str) -> State {
        let source = fs::read_to_string(filename).expect("Error reading file");
        let command = String::from(".|keys");
        let output = run_command(&command, filename);
        State {
            filename: String::from(filename),
            command_history: vec![command.as_str().to_string()],
            active_panel: ui::Panel::Command,
            source_pos: 0,
            output_pos: 0,
            command,
            output,
            source,

            history_pos: 0,
        }
    }

    pub fn run_current_command(&mut self) {
        self.command_history.push(self.command.to_string());
        self.history_pos = self.command_history.len() - 1;
        self.output = run_command(&self.command, self.filename.as_str());
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

    pub fn scroll_down(&mut self) {
        match self.active_panel {
            ui::Panel::Source => self.source_pos += 1,
            ui::Panel::Output => self.output_pos += 1,
            _ => {}
        };
    }

    pub fn scroll_up(&mut self) {
        match self.active_panel {
            ui::Panel::Source => {
                if self.source_pos > 0 {
                    self.source_pos -= 1;
                }
            }
            ui::Panel::Output => {
                if self.output_pos > 0 {
                    self.output_pos -= 1;
                }
            }
            _ => {}
        };
    }

    pub fn scroll_pos(&self, panel: ui::Panel) -> (u16, u16) {
        let x = 0;
        let y = match panel {
            ui::Panel::Source => self.source_pos,
            ui::Panel::Output => self.output_pos,
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
