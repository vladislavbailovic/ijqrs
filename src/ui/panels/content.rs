use super::super::super::{app, ui};
use crossterm::event::{KeyCode, KeyModifiers};

use super::Scroller;

pub struct Content {
    kind: ui::Panel,
    scroll: Scroller,
    content: String,
}
impl Content {
    pub fn new(content: String, kind: ui::Panel) -> Content {
        let s = Scroller::new(content.split('\n').count());
        Content {
            kind,
            scroll: s,
            content: String::from(content.as_str()),
        }
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
    fn handle_event(&mut self, _code: KeyCode, _modifiers: KeyModifiers) -> app::Signal {
        app::Signal::Nop
    }
}
