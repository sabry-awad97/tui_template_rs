use std::io::stdout;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use tui_template_rs::{
    event::{Event, EventHandler},
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
            Event::Key(key_event) => {}
            Event::Mouse(mouse_event) => {}
            Event::Resize(w, h) => {}
        }
    }
    Ok(())
}
