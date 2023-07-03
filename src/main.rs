use tui_template_rs::{App, AppResult};

fn main() -> AppResult<()> {
    let mut app = App::new();
    while app.is_running() {}
    Ok(())
}
