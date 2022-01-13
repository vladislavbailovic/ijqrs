use std::{
    fs,
    io::{self, BufRead},
};

use super::actions;
use super::ui;
use super::ui::Pane;

pub enum Signal {
    Quit,
    Nop,
    Run,
}

pub enum Mode {
    Internal,
    Shell,
    Help,
}

pub enum Status {
    Error(String),
    Ok,
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
            source += (line.expect("IO Error") + "\n").as_str();
        }
        let filename = actions::write_temp(&source);
        State::new(&filename, &source)
    }

    fn new(filename: &str, source: &str) -> State {
        let command = ui::panels::Command::new(String::from(".|keys"));
        let internal = ui::panels::Command::new(String::from(""));

        let mut state = State {
            filename: String::from(filename),
            source: ui::panels::Content::new(String::from(source), ui::Panel::Source),
            output: ui::panels::Content::new(String::from(""), ui::Panel::Output),

            command,
            internal,
            active: ui::Panel::Command,
            mode: Mode::Shell,
        };
        state.run_shell_command();
        state
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn switch_mode(&mut self) {
        match self.mode {
            Mode::Shell => self.set_mode(Mode::Internal),
            Mode::Internal => self.set_mode(Mode::Shell),
            Mode::Help => self.set_mode(Mode::Shell),
        };
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode
    }

    pub fn command(&self) -> &ui::panels::Command {
        match self.mode {
            Mode::Internal => &self.internal,
            Mode::Shell => &self.command,
            Mode::Help => &self.command,
        }
    }

    pub fn jq(&self) -> &ui::panels::Command {
        &self.command
    }

    pub fn get_mut_active(&mut self) -> Box<&mut dyn ui::Pane> {
        if ui::Panel::Source == self.active {
            return Box::new(&mut self.source);
        }
        if ui::Panel::Output == self.active {
            return Box::new(&mut self.output);
        }
        match self.mode {
            Mode::Internal => Box::new(&mut self.internal),
            Mode::Shell => Box::new(&mut self.command),
            Mode::Help => Box::new(&mut self.command),
        }
    }

    pub fn get_active(&self) -> Box<&dyn ui::Pane> {
        if ui::Panel::Source == self.active {
            return Box::new(&self.source);
        }
        if ui::Panel::Output == self.active {
            return Box::new(&self.output);
        }
        Box::new(self.command())
    }

    pub fn set_active(&mut self, active: ui::Panel) {
        self.active = active;
    }

    pub fn run_current_command(&mut self) {
        match self.mode {
            Mode::Shell => self.run_shell_command(),
            Mode::Internal => self.run_internal_command(),
            Mode::Help => (),
        }
    }

    pub fn run_internal_command(&mut self) {
        self.internal.record();
        let result = actions::run(&self.internal.get_content(), self);
        self.internal.clear();
        if let Err(msg) = result {
            self.internal.set_error(&msg);
        }
    }

    pub fn run_shell_command(&mut self) {
        self.command.record();
        let output = match actions::run(actions::RUN, self) {
            Ok(result) => result,
            Err(result) => result,
        };
        self.output = ui::panels::Content::new(output, ui::Panel::Output);
    }
}
