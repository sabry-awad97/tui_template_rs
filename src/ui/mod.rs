use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub struct TemplateRenderer;

impl TemplateRenderer {
    pub fn render<B: Backend>(_app: &mut App, frame: &mut Frame<'_, B>) {
        let title = "Template";
        let border_type = BorderType::Rounded;
        let borders = Borders::ALL;
        let title_alignment = Alignment::Center;
        let alignment = Alignment::Center;
        let fg_color = Color::Cyan;
        let bg_color = Color::Black;
        let text = "This is a tui template.";

        let block = Block::default()
            .title(title)
            .title_alignment(title_alignment)
            .borders(borders)
            .border_type(border_type);

        let style = Style::default().fg(fg_color).bg(bg_color);

        let paragraph = Paragraph::new(text)
            .block(block)
            .style(style)
            .alignment(alignment);

        frame.render_widget(paragraph, frame.size());
    }
}

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    TemplateRenderer::render(app, frame);
}
