use super::{app, help};
use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap, Clear},
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

    let content = state.source.get_content();
    let styled = get_styled(content.as_str(), state.source.highlight);
    let source_output = Paragraph::new(styled)
        .block(get_block(&Panel::Source, String::from("Source"), state))
        .scroll((state.source.get_pos() as u16, 0))
        .wrap(Wrap { trim: false });

    let content = state.output.get_content();
    let styled = get_styled(content.as_str(), state.output.highlight);
    let result_output = Paragraph::new(styled)
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

    if let app::Mode::Bookmarks = state.mode() {
        let padding = 3;
        let bm_area = Rect::new(
            frame_size.x + padding,
            frame_size.y + padding,
            frame_size.width - (padding * 2),
            frame_size.height - (padding * 2),
        );
        
        let bm_content = &state.bookmarks.get_content();
        let bm_items = get_styled(bm_content, state.bookmarks.get_pos() as usize);
        let bookmarks = Paragraph::new(bm_items)
            .block(get_block(&Panel::Bookmarks, "Bookmarks".to_string(), state))
            .wrap(Wrap { trim: false });
        frame.render_widget(Clear, bm_area);
        frame.render_widget(bookmarks, bm_area);
    }
}

fn get_styled(content: &str, index: usize) -> Vec<Spans> {
    let mut styled = Vec::new();
    for (idx, line) in content.split('\n').enumerate() {
        let mut style = Style::default();
        if idx == index {
            style = style.bg(COLOR_FG).fg(COLOR_BG);
        }
        styled.push(Spans::from(vec![Span::styled(line, style)]));
    }
    styled
}

fn get_block(panel: &Panel, title: String, state: &app::State) -> Block<'static> {
    let title = get_title(panel, title, state);
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
        Panel::Bookmarks => COLOR_FG_ACTIVE,
    };
    Block::default()
        .title(String::from(" ") + &title + " ")
        .borders(Borders::ALL)
        .style(Style::default().fg(fg).bg(COLOR_BG))
}

fn get_title(panel: &Panel, title: String, state: &app::State) -> String {
    match panel {
        Panel::Source => get_content_title(&state.source, title),
        Panel::Output => get_content_title(&state.output, title),
        _ => title,
    }
}

fn get_content_title(panel: &panels::Content, title: String) -> String {
    let mut title = title;
    {
        let suffix = match panel.mode {
            panels::content::PatternMode::None => String::from(""),
            panels::content::PatternMode::Receiving => {
                format!(": {}_", panel.pattern())
            }
            panels::content::PatternMode::Matching => {
                format!(": [{}] <{}>", panel.pattern(), panel.highlight)
            }
        };
        title = format!("{}{}", title, suffix);
    }
    title
}
