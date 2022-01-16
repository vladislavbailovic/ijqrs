use super::super::super::{app, ui};
use crossterm::event::{KeyCode, KeyModifiers};

use super::Scroller;

pub struct Content {
    kind: ui::Panel,
    pub mode: PatternMode,
    pattern: String,
    pub highlight: usize,
    scroll: Scroller,
    content: String,
}

impl Content {
    pub fn new(content: String, kind: ui::Panel) -> Content {
        let s = Scroller::new(content.split('\n').count());
        Content {
            kind,
            mode: PatternMode::None,
            pattern: String::from(""),
            highlight: 0,
            scroll: s,
            content: String::from(content.as_str()),
        }
    }

    pub fn pattern(&self) -> String {
        self.pattern.as_str().to_string()
    }

    fn push(&mut self, c: char) {
        self.pattern.push(c);
    }

    fn pop(&mut self) {
        self.pattern.pop();
    }

    fn find_next(&mut self) {
        for (count, line) in self.content.split('\n').enumerate() {
            if line.contains(&self.pattern) && count > self.scroll.get() {
                self.scroll.set_position(count);
                self.highlight = count;
                break;
            }
        }
    }

    fn find_prev(&mut self) {
        let mut lines: Vec<&str> = self.content.split('\n').collect();
        let mut count = lines.len();
        lines.reverse();
        for line in lines {
            count -= 1;
            if line.contains(&self.pattern) && count < self.scroll.get() {
                self.scroll.set_position(count);
                self.highlight = count;
                break;
            }
        }
    }

    fn reset_search(&mut self) {
        self.mode = PatternMode::None;
        self.pattern = String::from("");
        self.highlight = 0;
    }
}

impl ui::Pane for Content {
    fn scroll_up(&mut self) {
        self.scroll.prev();
    }

    fn scroll_down(&mut self) {
        self.scroll.next();
    }

    fn get_pos(&self) -> u16 {
        self.scroll.get() as u16
    }

    fn get_type(&self) -> &ui::Panel {
        &self.kind
    }

    fn get_content(&self) -> String {
        self.content.as_str().to_string()
    }

    fn handle_event(&mut self, code: KeyCode, modifiers: KeyModifiers) -> app::Signal {
        match code {
            KeyCode::Esc => {
                if let PatternMode::Matching = self.mode {
                    self.reset_search()
                }
            }
            KeyCode::Enter => match self.mode {
                PatternMode::Receiving => {
                    self.mode = PatternMode::Matching;
                    self.find_next();
                }
                PatternMode::Matching => {
                    self.find_next();
                }
                _ => (),
            },
            KeyCode::Backspace => {
                if let PatternMode::Receiving = self.mode {
                    self.pop()
                }
            }
            KeyCode::Char('/') => {
                if let PatternMode::None = self.mode {
                    self.mode = PatternMode::Receiving
                }
            }
            KeyCode::Char('l') => {
                if modifiers == KeyModifiers::CONTROL {
                    self.reset_search()
                } else if let PatternMode::Receiving = self.mode {
                    self.push('l');
                }
            }
            KeyCode::Char('n') => match self.mode {
                PatternMode::Receiving => self.push('n'),
                PatternMode::Matching => {
                    self.find_next();
                }
                _ => (),
            },
            KeyCode::Char('N') => match self.mode {
                PatternMode::Receiving => self.push('N'),
                PatternMode::Matching => {
                    self.find_prev();
                }
                _ => (),
            },
            KeyCode::Char(c) => {
                if let PatternMode::Receiving = self.mode {
                    self.push(c)
                }
            }
            _ => (),
        };
        app::Signal::Nop
    }
}

pub enum PatternMode {
    None,
    Receiving,
    Matching,
}
