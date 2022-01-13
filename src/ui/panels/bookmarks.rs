use super::super::super::{app, ui};
use crossterm::event::{KeyCode, KeyModifiers};

use super::Scroller;

pub struct Bookmarks {
    items: Vec<String>,
    scroll: Scroller,
}

impl Bookmarks {
    pub fn new() -> Self {
        Self{
            items: Vec::new(),
            scroll: Scroller::new(0),
        }
    }

    pub fn add(&mut self, bm: &str) {
        self.items.push(String::from(bm))
    }

    pub fn items(&self) -> &Vec<String> {
        &self.items()
    }

    fn del(&mut self, idx: usize) -> bool {
        if idx < self.items.len() {
            self.items.remove(idx);
            return true;
        }
        false
    }
}

impl ui::Pane for Bookmarks {
    fn get_pos(&self) -> u16 {
        self.scroll.get() as u16
    }
    fn scroll_up(&mut self) {
        self.scroll.prev();
    }
    fn scroll_down(&mut self) {
        self.scroll.next();
    }
    fn get_type(&self) -> &ui::Panel {
        &ui::Panel::Bookmarks
    }

    fn get_content(&self) -> String {
        let mut out = String::new();
        for bm in &self.items {
            let item = format!("\t{}\n", bm);
            out.push_str(&item);
        }
        out
    }

    fn handle_event(&mut self, code: KeyCode, _modifiers: KeyModifiers) -> app::Signal {
        match code {
            KeyCode::Enter => {
                todo!("Handle bookmark selection");
            },
            _ => return app::Signal::Nop,
        };
        app::Signal::Nop
    }
}
