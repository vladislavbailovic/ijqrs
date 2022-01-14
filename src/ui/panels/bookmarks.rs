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
        let mut items = Vec::new();
        for item in load_bookmarks() {
            items.push(item);
        }
        Self {
            items,
            scroll: Scroller::new(0),
        }
    }

    pub fn add(&mut self, bm: &str) {
        if !self.items.contains(&String::from(bm)) {
            self.items.push(String::from(bm));
            save_bookmarks(&self.items);
        }
        self.scroll.set_max(self.items.len() - 1);
    }

    pub fn items(&self) -> &Vec<String> {
        &self.items
    }

    pub fn get_current_item(&self) -> String {
        self.get(self.scroll.get())
    }

    pub fn del_current_item(&mut self) -> bool {
        self.del(self.scroll.get())
    }

    fn get(&self, idx: usize) -> String {
        if idx < self.items.len() {
            return self.items[idx].as_str().to_string();
        }
        "".to_string()
    }

    fn del(&mut self, idx: usize) -> bool {
        if idx < self.items.len() {
            self.items.remove(idx);
            save_bookmarks(&self.items);
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

    fn handle_event(&mut self, code: KeyCode, modifiers: KeyModifiers) -> app::Signal {
        match code {
            KeyCode::Enter => app::Signal::LoadBookmark(self.get_current_item()),
            KeyCode::Delete => {
                self.del_current_item();
                app::Signal::Nop
            }
            KeyCode::Char('d') => {
                if KeyModifiers::CONTROL == modifiers {
                    self.del_current_item();
                }
                app::Signal::Nop
            }
            _ => app::Signal::Nop,
        }
    }
}

use std::io::Write;
use std::path::{Path, PathBuf};

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("Unable to resolve user home directory");
    let path = Path::new(&home).join(".config").join("ijqrs");

    if !path.exists() {
        let dir = path
            .to_str()
            .expect("Unable to resolve the user config directory");
        std::fs::create_dir_all(dir).expect("Unable to create the missing config directory");
    }

    path
}

fn get_bookmarks_file_path() -> String {
    let path = get_config_path().join("bookmarks");
    if !path.exists() {
        std::fs::File::create(&path).expect("Unable to create the bookmarks file");
    }
    path.to_str()
        .expect("Unable to resolve bookmarks file")
        .to_string()
}

fn load_bookmarks() -> Vec<String> {
    let mut result = Vec::new();
    let bm_file = get_bookmarks_file_path();
    let raw = std::fs::read_to_string(bm_file).expect("Unable to read bookmarks file");
    for line in raw.split('\n') {
        if !line.is_empty() {
            result.push(line.to_string());
        }
    }
    result
}

fn save_bookmarks(bms: &[String]) {
    let mut bookmarks = String::new();
    for bm in bms {
        bookmarks.push_str(&format!("{}\n", bm));
    }
    let fpath = get_bookmarks_file_path();
    let mut file = std::fs::File::create(fpath).expect("Unable to create the bookmarks file");
    file.write_all(bookmarks.as_bytes())
        .expect("Error writing bookmarks file");
}
