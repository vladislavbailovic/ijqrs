use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::app;

#[derive(Debug, PartialEq)]
pub enum Panel {
    Source,
    Output,
    Command,
}

const COLOR_BG: Color = Color::Black;
const COLOR_FG: Color = Color::Gray;
const COLOR_FG_ACTIVE: Color = Color::White;

pub fn draw<B: Backend>(frame: &mut Frame<B>, state: &mut app::State) {
    let frame_size = frame.size();
    let half_width = frame_size.width / 2;
    let vert_height = frame_size.height - 3;
    let source_size = Rect::new(frame_size.x, frame_size.y, half_width, vert_height);
    let result_size = Rect::new(
        frame_size.x + half_width,
        frame_size.y,
        half_width,
        vert_height,
    );
    let cmd_size = Rect::new(
        frame_size.x,
        frame_size.y + vert_height,
        frame_size.width,
        frame_size.height - vert_height,
    );

    let source_output = Paragraph::new(state.source.get_content())
        .block(get_block(&Panel::Source, String::from("Source"), state))
        .scroll(state.scroll_pos(Panel::Source))
        .wrap(Wrap { trim: true });
    let result_output = Paragraph::new(state.output.get_content())
        .block(get_block(&Panel::Output, String::from("Result"), state))
        .scroll(state.scroll_pos(Panel::Output))
        .wrap(Wrap { trim: true });
    let cmd_output = Paragraph::new(state.command.get_content())
        .block(get_block(&Panel::Command, String::from("Command"), state))
        .wrap(Wrap { trim: true });

    frame.render_widget(source_output, source_size);
    frame.render_widget(result_output, result_size);
    frame.render_widget(cmd_output, cmd_size);
}

fn get_block(panel: &Panel, title: String, state: &app::State) -> Block<'static> {
    let fg: Color = match panel {
        Panel::Source => match state.get_active().get_type() {
            Panel::Source => COLOR_FG_ACTIVE,
            _ => COLOR_FG,
        },
        Panel::Output => match state.get_active().get_type() {
            Panel::Output => COLOR_FG_ACTIVE,
            _ => COLOR_FG,
        },
        Panel::Command => match state.get_active().get_type() {
            Panel::Command => COLOR_FG_ACTIVE,
            _ => COLOR_FG,
        },
    };
    Block::default()
        .title(String::from(" ") + &title + " ")
        .borders(Borders::ALL)
        .style(Style::default().fg(fg).bg(COLOR_BG))
}

pub struct Scroller {
    position: usize,
    max: usize
}
impl Scroller {

    pub fn new(max: usize) -> Scroller {
        Scroller{ position: 0, max }
    }

    pub fn prev(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    pub fn next(&mut self) {
        if self.position < self.max {
            self.position += 1;
        }
    }

    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }

    pub fn get(&self) -> usize {
        self.position
    }

}

pub trait Pane {
    fn scroll_up(&mut self);
    fn scroll_down(&mut self);
    fn get_pos(&self) -> u16;
    fn get_content(&self) -> String;
    fn get_type(&self) -> &Panel;
}

pub struct ContentPanel {
    kind: Panel,
    scroll: Scroller,
    content: String
}
impl ContentPanel {
    pub fn new(content: String, kind: Panel) -> ContentPanel {
        let lines: Vec<&str> = content.split("\n").collect();
        let s = Scroller::new(lines.len());
        ContentPanel{
            kind: kind,
            scroll: s,
            content: String::from(content.as_str())
        }
    }
}
impl Pane for ContentPanel {
    fn scroll_up(&mut self) { self.scroll.prev(); }
    fn scroll_down(&mut self) { self.scroll.next(); }
    fn get_pos(&self) -> u16 { self.scroll.get() as u16 }
    fn get_type(&self) -> &Panel { &self.kind }
    fn get_content(&self) -> String {
       self.content.as_str().to_string()
    }
}

pub struct CommandPanel {
    scroll: Scroller,
    history: Vec<String>,
    command: String
}
impl CommandPanel {
    pub fn new(command: String) -> CommandPanel{
        let s = Scroller::new(0);
        CommandPanel{
            scroll: s,
            history: vec![command.as_str().to_string()],
            command: command
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

    pub fn push(&mut self, c: char) {
        self.command.push(c);
    }

    pub fn pop(&mut self) {
        self.command.pop();
    }

    pub fn record(&mut self) {
        self.history.push(self.command.to_string());
        self.scroll.set_max(self.history.len() - 1);
        self.scroll.position = self.scroll.max;
    }
}
impl Pane for CommandPanel {
    fn get_pos(&self) -> u16 { self.scroll.get() as u16 }
    fn get_content(&self) -> String {
       self.command.as_str().to_string()
    }
    fn get_type(&self) -> &Panel { &Panel::Command }
    fn scroll_up(&mut self) { self.prev_from_history(); }
    fn scroll_down(&mut self) { self.next_from_history(); }
}
