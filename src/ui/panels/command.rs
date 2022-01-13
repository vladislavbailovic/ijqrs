use super::super::super::{app, ui};
use crossterm::event::{KeyCode, KeyModifiers};

use super::Scroller;

pub struct Command {
    scroll: Scroller,
    cursor: Scroller,
    history: Vec<String>,
    command: String,
    status: app::Status,
}

impl Command {
    pub fn new(command: String) -> Command {
        let s = Scroller::new(0);
        let mut c = Scroller::new(0);
        c.set_max(command.len());
        c.set_position(command.len());
        Command {
            scroll: s,
            cursor: c,
            history: vec![command.as_str().to_string()],
            command,
            status: app::Status::Ok,
        }
    }

    pub fn clear(&mut self) {
        self.command = String::from("");
        self.tail_cursor();
    }

    pub fn record(&mut self) {
        self.history.push(self.command.to_string());
        self.scroll.set_max(self.history.len() - 1);
        self.scroll.set_position(self.scroll.max());
    }

    pub fn set_error(&mut self, error: &str) {
        self.status = app::Status::Error(error.to_string());
    }

    pub fn cursor(&self) -> usize {
        self.cursor.get()
    }

    pub fn replace(&mut self, with: String) {
        self.command = with;
    }

    fn prev_from_history(&mut self) {
        self.scroll.prev();
        self.command = self.history[self.scroll.get()].as_str().to_string();
    }

    fn next_from_history(&mut self) {
        self.scroll.next();
        self.command = self.history[self.scroll.get()].as_str().to_string();
    }

    fn push(&mut self, c: char) {
        self.clear_error();
        let cur = self.cursor.get();
        if cur == self.command.len() {
            self.command.push(c);
            self.tail_cursor();
        } else {
            let mut newcmd: Vec<char> = Vec::new();
            for (idx, old) in self.command.chars().enumerate() {
                if idx == cur {
                    newcmd.push(c);
                }
                newcmd.push(old);
            }
            self.command = newcmd.into_iter().collect();
            self.cursor.set_max(self.command.len());
            self.cursor.next();
        }
    }

    fn delete(&mut self) {
        self.clear_error();
        let cur = self.cursor.get();
        let mut newcmd: Vec<char> = Vec::new();
        for (idx, old) in self.command.chars().enumerate() {
            if idx == cur {
                continue;
            }
            newcmd.push(old);
        }
        self.command = newcmd.into_iter().collect();
        self.cursor.set_max(self.command.len());
    }

    fn backspace(&mut self) {
        self.clear_error();
        let cur = self.cursor.get();
        if cur == self.command.len() {
            self.command.pop();
            self.tail_cursor();
        } else {
            let mut newcmd: Vec<char> = Vec::new();
            for (idx, old) in self.command.chars().enumerate() {
                if idx == cur - 1 {
                    continue;
                }
                newcmd.push(old);
            }
            self.command = newcmd.into_iter().collect();
            self.cursor.set_max(self.command.len());
            self.cursor.prev();
        }
    }

    fn tail_cursor(&mut self) {
        self.cursor.set_max(self.command.len());
        self.cursor.set_position(self.command.len());
    }

    fn clear_error(&mut self) {
        self.status = app::Status::Ok;
    }
}

impl ui::Pane for Command {
    fn get_pos(&self) -> u16 {
        self.scroll.get() as u16
    }
    fn get_content(&self) -> String {
        match &self.status {
            app::Status::Error(err) => err.as_str().to_string(),
            app::Status::Ok => self.command.as_str().to_string(),
        }
    }
    fn get_type(&self) -> &ui::Panel {
        &ui::Panel::Command
    }
    fn scroll_up(&mut self) {
        self.prev_from_history();
    }
    fn scroll_down(&mut self) {
        self.next_from_history();
    }

    fn handle_event(&mut self, code: KeyCode, _modifiers: KeyModifiers) -> app::Signal {
        match code {
            KeyCode::Char(c) => {
                self.push(c);
            }
            KeyCode::Backspace => {
                self.backspace();
            }
            KeyCode::Delete => {
                self.delete();
            }
            KeyCode::Enter => {
                return app::Signal::Run;
            }
            KeyCode::Left => {
                self.cursor.prev();
            }
            KeyCode::Home => {
                self.cursor.set_position(0);
            }
            KeyCode::Right => {
                self.cursor.next();
            }
            KeyCode::End => {
                self.tail_cursor();
            }
            _ => return app::Signal::Nop,
        };
        app::Signal::Nop
    }
}
