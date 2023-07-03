use tui::{backend::Backend, Frame};

use crate::App;

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {}
