use std::process::Command;
use std::{fs, io};

use tui::{
    {Terminal,Frame},
    backend::{Backend,CrosstermBackend},
    widgets::{Block, Borders, Wrap, Paragraph},
    layout::Rect,
    style::{Style, Color}
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode}
};

const COLOR_BG: Color = Color::Black;
const COLOR_FG: Color = Color::Gray;
const COLOR_FG_ACTIVE: Color = Color::White;

#[derive(Debug)]
enum Panel {
    Source,
    Output,
    Command
}

struct State {
    command: String,
    output: String,
    source: String,
    active_panel: Panel,
    source_pos: usize,
    output_pos: usize
}

fn main() {
    let filename = "./examples/generated.json";
    let source = fs::read_to_string(filename)
        .expect("Error reading file");
    let command = String::from(".|keys");
    let output = run_command(&command);
    let app: &mut State = &mut State{
        active_panel: Panel::Output,
        source_pos: 0, output_pos: 0,
        command, output, source };

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Unable to bootstrap terminal");

    enable_raw_mode();
    loop {
        terminal.draw(|frame| draw_ui(frame, app));
        if let Ok(Event::Key(key)) = event::read() {
            match key {
                KeyEvent{
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL
                } => { disable_raw_mode(); return },
                KeyEvent{
                    code: KeyCode::Char('w'),
                    modifiers: KeyModifiers::CONTROL
                } => {
                    app.active_panel = match app.active_panel {
                        Panel::Output => Panel::Source,
                        _ => Panel::Output,
                    };
                },
                KeyEvent{
                    code,
                    modifiers
                } => {
                    match code {
                        KeyCode::Char(c) => { app.command.push(c); },
                        KeyCode::Backspace => { app.command.pop(); },
                        KeyCode::Enter => { app.output = run_command(&app.command) }
                        KeyCode::Down => match app.active_panel {
                            Panel::Source => app.source_pos += 1,
                            Panel::Output => app.output_pos += 1,
                            _ => continue,
                        },
                        KeyCode::Up => match app.active_panel {
                            Panel::Source => if app.source_pos > 0 {
                                app.source_pos -= 1;
                            },
                            Panel::Output => if app.output_pos > 0 {
                                app.output_pos -= 1;
                            },
                            _ => continue,
                        },
                        _ => continue
                    }
                },
                _ => continue
            };
        }
    };
}

fn get_block(panel: &Panel, title: String, app: &State) -> Block<'static> {
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

fn run_command(command: &str) -> String {
    let command = Command::new("jq")
            .arg(command)
            .arg("./examples/generated.json")
            .output().expect("Command execution failed");
    let result = String::from_utf8(command.stdout).expect("Invalid stdout");
    if "" == result {
        return String::from_utf8(command.stderr).expect("Invalid stdout");
    }
    result
}

fn draw_ui<B: Backend>(frame: &mut Frame<B>, app: &mut State) {
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
