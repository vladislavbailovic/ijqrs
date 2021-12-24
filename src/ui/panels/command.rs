use crossterm::event::{KeyCode, KeyModifiers};
use super::super::super::{app,ui};

use super::Scroller;

pub struct Command {
    scroll: Scroller,
    history: Vec<String>,
    command: String,
    output: String,
    status: app::Status,
}

impl Command {
    pub fn new(command: String) -> Command{
        let s = Scroller::new(0);
        Command{
            scroll: s,
            history: vec![command.as_str().to_string()],
            command: command,
            output: String::from(""),
            status: app::Status::Ok,
        }
    }

    pub fn prev_from_history(&mut self) {
        self.scroll.prev();
        self.command = self.history[self.scroll.get()].as_str().to_string();
    }

    pub fn next_from_history(&mut self) {
        self.scroll.next();
        self.command = self.history[self.scroll.get()].as_str().to_string();
    }

    pub fn clear(&mut self) {
        self.command = String::from("");
    }

    pub fn push(&mut self, c: char) {
        match self.status {
            app::Status::Error => self.set_output(""),
            app::Status::Ok => (),
        };
        self.command.push(c);
    }

    pub fn pop(&mut self) {
        match self.status {
            app::Status::Error => self.set_output(""),
            app::Status::Ok => (),
        };
        self.command.pop();
    }

    pub fn record(&mut self) {
        self.history.push(self.command.to_string());
        self.scroll.set_max(self.history.len() - 1);
        self.scroll.set_position(self.scroll.max());
    }

    pub fn set_error(&mut self, error: &str) {
        self.output = String::from(error);
        self.status = app::Status::Error;
    }

    pub fn set_output(&mut self, output: &str) {
        self.output = String::from(output);
        self.status = app::Status::Ok;
    }
}

impl ui::Pane for Command {
    fn get_pos(&self) -> u16 { self.scroll.get() as u16 }
    fn get_content(&self) -> String {
        match self.status {
            app::Status::Error => self.output.as_str().to_string(),
            app::Status::Ok => self.command.as_str().to_string(),
        }
    }
    fn get_type(&self) -> &ui::Panel { &ui::Panel::Command }
    fn scroll_up(&mut self) { self.prev_from_history(); }
    fn scroll_down(&mut self) { self.next_from_history(); }

    fn handle_event(&mut self, code: KeyCode, _modifiers: KeyModifiers) -> app::Signal {
        match code {
            KeyCode::Char(c) => {
                self.push(c);
            },
            KeyCode::Backspace => {
                self.pop();
            },
            KeyCode::Enter => {
                return app::Signal::Run;
            },
            _ => return app::Signal::Nop,
        };
        app::Signal::Nop
    }
}

