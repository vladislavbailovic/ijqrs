use super::{app, help};
use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
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
    let hparts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Min(1)].as_ref())
        .split(frame.size());
    let mut parts = Vec::new();
    for hp in &hparts {
        let vparts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Min(1)].as_ref())
            .split(*hp);
        for vp in vparts {
            parts.push(vp);
        }
    }

    let hlp = help::get_help();
    for (idx, title) in hlp.keys().enumerate() {
        let help_string = &hlp[title.as_str()].join("\n");
        let output = Paragraph::new(help_string.to_string())
            .block(get_block(&Panel::Help, title, state))
            .wrap(Wrap { trim: false });
        frame.render_widget(output, parts[idx]);
    }
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
        .block(get_block(&Panel::Source, "Source", state))
        .scroll((state.source.get_pos(), 0))
        .wrap(Wrap { trim: false });

    let content = state.output.get_content();
    let styled = get_styled(content.as_str(), state.output.highlight);
    let result_output = Paragraph::new(styled)
        .block(get_block(&Panel::Output, "Result", state))
        .scroll((state.output.get_pos(), 0))
        .wrap(Wrap { trim: false });

    let cmd_title = match *state.mode() {
        app::Mode::Shell => "jq Command",
        app::Mode::Internal => "Internal Command",
        _ => "",
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
            .block(get_block(&Panel::Bookmarks, "Bookmarks", state))
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

fn get_block(panel: &Panel, title: &str, state: &app::State) -> Block<'static> {
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

fn get_title(panel: &Panel, title: &str, state: &app::State) -> String {
    match panel {
        Panel::Source => state.source.get_title(&title),
        Panel::Output => state.output.get_title(&title),
        _ => title.to_string(),
    }
}