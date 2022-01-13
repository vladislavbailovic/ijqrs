use super::super::super::{app, ui};
use crossterm::event::{KeyCode, KeyModifiers};

use super::Scroller;

pub struct Bookmarks {
    items: Vec<String>,
    scroll: Scroller,
}

impl Default for Bookmarks {
    fn default() -> Self {
        Self::new()
    }
}

impl Bookmarks {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            scroll: Scroller::new(0),
        }
    }

    pub fn add(&mut self, bm: &str) {
        if !self.items.contains(&String::from(bm)) {
            self.items.push(String::from(bm));
        }
        self.scroll.set_max(self.items.len() - 1);
    }

    pub fn items(&self) -> &Vec<String> {
        &self.items
    }

    pub fn get_current_item(&self) -> String {
        self.get(self.scroll.get())
    }

    fn get(&self, idx: usize) -> String {
        if idx < self.items.len() {
            return self.items[idx].as_str().to_string();
        }
        "".to_string()
    }

    fn _del(&mut self, idx: usize) -> bool {
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
            KeyCode::Enter => app::Signal::LoadBookmark(self.get_current_item()),
            // TODO: delete bookmark
            // TODO: persist bookmarks
            _ => app::Signal::Nop,
        }
    }
}
