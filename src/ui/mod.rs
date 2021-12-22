use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crossterm::event::{KeyCode, KeyModifiers};
use super::app;

pub mod panels;
pub mod scroller;

pub use scroller::Scroller;

#[derive(Debug, PartialEq)]
pub enum Panel {
    Source,
    Output,
    Command,
}

pub trait Pane {
    fn scroll_up(&mut self);
    fn scroll_down(&mut self);
    fn get_pos(&self) -> u16;
    fn get_content(&self) -> String;
    fn get_type(&self) -> &Panel;
    fn handle_event(&mut self, code: KeyCode, modifiers: KeyModifiers) -> app::Signal;
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
    let cmd_title = match state.mode() {
        &app::Mode::Shell => String::from("jq Command"),
        &app::Mode::Internal => String::from("Internal Command"),
    };
    let cmd_output = Paragraph::new(state.command().get_content())
        .block(get_block(&Panel::Command, cmd_title, state))
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
