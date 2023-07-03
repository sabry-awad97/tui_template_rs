use crate::app::{App, AppResult};
use termimad::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Right => {}
        KeyCode::Left => {}
        _ => {}
    }
    Ok(())
}

pub fn handle_mouse_events(_mouse_event: MouseEvent, _app: &mut App) -> AppResult<()> {
    Ok(())
}

pub fn handle_resize_events((_width, _height): (u16, u16), _app: &mut App) -> AppResult<()> {
    Ok(())
}
