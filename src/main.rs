use std::io::stdout;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use tui_template_rs::{
    event::{handle_key_events, handle_mouse_events, handle_resize_events, Event, EventHandler},
    tui::Tui,
    App, AppResult,
};

fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::default();
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.is_running() {
        tui.update(&mut app)?;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(w, h) => handle_resize_events((w, h), &mut app)?,
        }
    }

    tui.exit()?;
    Ok(())
}
