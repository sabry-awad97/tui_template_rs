use std::io::stdout;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use tui_template_rs::{App, AppResult};

fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    

    while app.is_running() {}
    Ok(())
}
