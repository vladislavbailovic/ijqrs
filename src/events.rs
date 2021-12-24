use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

pub fn global_keys() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("Ctrl+q", "Quit"),
        ("Ctrl+w", "Switch panel"),
        ("Esc", "Switch mode"),
        ("?", "Show help"),
    ])
}

use super::app;
use super::ui;

fn handle_key_event(key: KeyEvent, state: &mut app::State) -> app::Signal {
    match key {
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
        } => app::Signal::Quit,
        KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            let active = match state.get_active().get_type() {
                ui::Panel::Command => ui::Panel::Output,
                ui::Panel::Output => ui::Panel::Source,
                ui::Panel::Source => ui::Panel::Command,
                ui::Panel::Help => ui::Panel::Help,
            };
            state.set_active(active);
            app::Signal::Nop
        }
        KeyEvent { code, modifiers } => match code {
            KeyCode::Esc => {
                state.switch_mode();
                state.set_active(ui::Panel::Command);
                return app::Signal::Nop;
            }
            KeyCode::Char('?') => {
                state.set_mode(app::Mode::Help);
                return app::Signal::Nop;
            }
            KeyCode::Down => {
                state.get_mut_active().scroll_down();
                return app::Signal::Nop;
            }
            KeyCode::Up => {
                state.get_mut_active().scroll_up();
                return app::Signal::Nop;
            }
            _ => state.get_mut_active().handle_event(code, modifiers),
        },
    }
}

pub fn handler(state: &mut app::State) -> app::Signal {
    if let Ok(Event::Key(key)) = event::read() {
        return handle_key_event(key, state);
    }
    app::Signal::Nop
}
