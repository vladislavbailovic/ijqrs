use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, KeyEvent}
};

use super::app;
use super::ui;

fn handle_key_event(key: KeyEvent, state: &mut app::State) -> app::Signal {
    match key {
        KeyEvent{
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL
        } => app::Signal::Quit,
        KeyEvent{
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::CONTROL
        } => {
            state.active_panel = match state.active_panel {
                ui::Panel::Command => ui::Panel::Output,
                ui::Panel::Output => ui::Panel::Source,
                ui::Panel::Source => ui::Panel::Command
            };
            return app::Signal::Nop;
        },
        KeyEvent{
            code,
            modifiers
        } => handle_key_code(code, modifiers, state)
    }
}

fn handle_key_code(code: KeyCode, modifiers: KeyModifiers, state: &mut app::State) -> app::Signal {
    match code {
        KeyCode::Char(c) => { state.command.push(c); },
        KeyCode::Backspace => { state.command.pop(); },
        KeyCode::Enter => { state.run_current_command(); }
        KeyCode::Down => match state.active_panel {
            ui::Panel::Source => state.source_pos += 1,
            ui::Panel::Output => state.output_pos += 1,
            ui::Panel::Command => state.next_from_history()
        },
        KeyCode::Up => match state.active_panel {
            ui::Panel::Source => if state.source_pos > 0 {
                state.source_pos -= 1;
            },
            ui::Panel::Output => if state.output_pos > 0 {
                state.output_pos -= 1;
            },
            ui::Panel::Command => state.prev_from_history()
        },
        _ => return app::Signal::Nop
    };
    return app::Signal::Nop;
}

pub fn handler(state: &mut app::State) -> app::Signal {
    if let Ok(Event::Key(key)) = event::read() {
        return handle_key_event(key, state);
    }
    return app::Signal::Nop;
}
