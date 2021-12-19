use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, KeyEvent}
};

use super::app;
use super::ui;

pub fn handler(app: &mut app::State) -> app::Signal {
    if let Ok(Event::Key(key)) = event::read() {
        match key {
            KeyEvent{
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL
            } => { return app::Signal::Quit; },
            KeyEvent{
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::CONTROL
            } => {
                app.active_panel = match app.active_panel {
                    ui::Panel::Output => ui::Panel::Source,
                    _ => ui::Panel::Output,
                };
            },
            KeyEvent{
                code,
                modifiers
            } => {
                match code {
                    KeyCode::Char(c) => { app.command.push(c); },
                    KeyCode::Backspace => { app.command.pop(); },
                    KeyCode::Enter => { app.output = app::run_command(&app.command) }
                    KeyCode::Down => match app.active_panel {
                        ui::Panel::Source => app.source_pos += 1,
                        ui::Panel::Output => app.output_pos += 1,
                        _ => return app::Signal::Nop,
                    },
                    KeyCode::Up => match app.active_panel {
                        ui::Panel::Source => if app.source_pos > 0 {
                            app.source_pos -= 1;
                        },
                        ui::Panel::Output => if app.output_pos > 0 {
                            app.output_pos -= 1;
                        },
                        _ => return app::Signal::Nop,
                    },
                    _ => return app::Signal::Nop
                }
            },
            _ => return app::Signal::Nop
        };
    }
    return app::Signal::Nop;
}
