use crossterm::event::{KeyCode, KeyModifiers};

pub mod content;
pub use content::Content;

pub mod command;
pub use command::Command;

pub mod bookmarks;
pub use bookmarks::Bookmarks;

pub use super::{super::app, scroller::Scroller};

#[derive(Debug, PartialEq)]
pub enum Panel {
    Source,
    Output,
    Command,
    Bookmarks,
    Help,
}

pub trait Pane {
    fn scroll_up(&mut self);
    fn scroll_down(&mut self);
    fn get_pos(&self) -> u16;
    fn get_content(&self) -> String;
    fn get_type(&self) -> &Panel;
    fn handle_event(&mut self, code: KeyCode, modifiers: KeyModifiers) -> app::Signal;
}
