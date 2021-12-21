use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::app;

#[derive(Debug)]
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

    let source_output = Paragraph::new(state.source.as_str())
        .block(get_block(&Panel::Source, String::from("Source"), state))
        .scroll(state.scroll_pos(Panel::Source))
        .wrap(Wrap { trim: true });
    let result_output = Paragraph::new(state.output.as_str())
        .block(get_block(&Panel::Output, String::from("Result"), state))
        .scroll(state.scroll_pos(Panel::Output))
        .wrap(Wrap { trim: true });
    let cmd_output = Paragraph::new(state.command.as_str())
        .block(get_block(&Panel::Command, String::from("Command"), state))
        .wrap(Wrap { trim: true });

    frame.render_widget(source_output, source_size);
    frame.render_widget(result_output, result_size);
    frame.render_widget(cmd_output, cmd_size);
}

fn get_block(panel: &Panel, title: String, state: &app::State) -> Block<'static> {
    let fg: Color = match panel {
        Panel::Source => match state.active_panel {
            Panel::Source => COLOR_FG_ACTIVE,
            _ => COLOR_FG,
        },
        Panel::Output => match state.active_panel {
            Panel::Output => COLOR_FG_ACTIVE,
            _ => COLOR_FG,
        },
        Panel::Command => match state.active_panel {
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
