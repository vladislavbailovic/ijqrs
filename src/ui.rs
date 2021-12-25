use super::{app, help};
use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub mod panels;
pub mod scroller;

pub use scroller::Scroller;

#[derive(Debug, PartialEq)]
pub enum Panel {
    Source,
    Output,
    Command,
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

const COLOR_BG: Color = Color::Black;
const COLOR_FG: Color = Color::Gray;
const COLOR_FG_ACTIVE: Color = Color::White;

pub fn draw<B: Backend>(frame: &mut Frame<B>, state: &mut app::State) {
    match state.mode() {
        app::Mode::Help => draw_help(frame, state),
        _ => draw_app(frame, state),
    }
}

pub fn draw_help<B: Backend>(frame: &mut Frame<B>, state: &mut app::State) {
    let output = Paragraph::new(help::usage() + "\n" + help::shortcuts().as_str())
        .block(get_block(&Panel::Help, String::from("Help"), state))
        .wrap(Wrap { trim: false });
    frame.render_widget(output, frame.size());
}

pub fn draw_app<B: Backend>(frame: &mut Frame<B>, state: &mut app::State) {
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
        .scroll((state.source.get_pos() as u16, 0))
        .wrap(Wrap { trim: false });
    let result_output = Paragraph::new(state.output.get_content())
        .block(get_block(&Panel::Output, String::from("Result"), state))
        .scroll((state.output.get_pos() as u16, 0))
        .wrap(Wrap { trim: false });

    let cmd_title = match *state.mode() {
        app::Mode::Shell => String::from("jq Command"),
        app::Mode::Internal => String::from("Internal Command"),
        _ => String::from(""),
    };
    let mut cmd = state.command().get_content();
    let cursor = state.command().cursor();
    if cursor == cmd.len() {
        cmd += "_";
    }
    let mut editable = Vec::new();
    for (idx, c) in cmd.chars().enumerate() {
        let mut style = Style::default();
        if idx == cursor {
            style = style.bg(COLOR_FG).fg(COLOR_BG);
        }
        editable.push(Span::styled(String::from(c), style));
    }
    let cmd_output = Paragraph::new(Spans::from(editable))
        .block(get_block(&Panel::Command, cmd_title, state))
        .wrap(Wrap { trim: false });

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
        Panel::Help => COLOR_FG_ACTIVE,
    };
    Block::default()
        .title(String::from(" ") + &title + " ")
        .borders(Borders::ALL)
        .style(Style::default().fg(fg).bg(COLOR_BG))
}