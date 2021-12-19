use tui::{
    Frame,
    backend::Backend,
    widgets::{Block, Borders, Wrap, Paragraph},
    layout::Rect,
    style::{Style, Color}
};

use super::app;

#[derive(Debug)]
pub enum Panel {
    Source,
    Output,
    Command
}

const COLOR_BG: Color = Color::Black;
const COLOR_FG: Color = Color::Gray;
const COLOR_FG_ACTIVE: Color = Color::White;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut app::State) {
    let frame_size = frame.size();
    let half_width = frame_size.width / 2;
    let vert_height = frame_size.height - 3;
    let source_size = Rect::new(
        frame_size.x,
        frame_size.y,
        half_width,
        vert_height
    );
    let result_size = Rect::new(
        frame_size.x + half_width,
        frame_size.y,
        half_width,
        vert_height
    );
    let cmd_size = Rect::new(
        frame_size.x,
        frame_size.y + vert_height,
        frame_size.width,
        frame_size.height - vert_height
    );

    let source_output = Paragraph::new(app.source.as_str())
        .block(get_block(&Panel::Source, String::from("Source"), app))
        .scroll((app.source_pos as u16, 0))
        .wrap(Wrap { trim: true });
    let result_output = Paragraph::new(app.output.as_str())
        .block(get_block(&Panel::Output, String::from("Result"), app))
        .scroll((app.output_pos as u16, 0))
        .wrap(Wrap { trim: true });
    let cmd_output = Paragraph::new(app.command.as_str())
        .block(get_block(&Panel::Command, String::from("Command"), app))
        .wrap(Wrap { trim: true });

    frame.render_widget(source_output, source_size);
    frame.render_widget(result_output, result_size);
    frame.render_widget(cmd_output, cmd_size);
}

fn get_block(panel: &Panel, title: String, app: &app::State) -> Block<'static> {
    let fg: Color = match panel {
        Panel::Source => match app.active_panel {
            Panel::Source => COLOR_FG_ACTIVE,
            _ => COLOR_FG,
        },
        Panel::Output => match app.active_panel {
            Panel::Output => COLOR_FG_ACTIVE,
            _ => COLOR_FG,
        },
        _ => COLOR_FG_ACTIVE
    };
    Block::default().title(String::from(" ") + &title + " ")
        .borders(Borders::ALL)
        .style(Style::default().fg(fg).bg(COLOR_BG))
}
