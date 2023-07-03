use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render<B: Backend>(_app: &mut App, frame: &mut Frame<'_, B>) {
    frame.render_widget(
        Paragraph::new("This is a tui template.")
            .block(
                Block::default()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        frame.size(),
    )
}
