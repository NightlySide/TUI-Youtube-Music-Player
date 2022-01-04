use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

use crate::app::{App, Screen};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let title_block = Block::default().title("Playlists").borders(Borders::ALL);
    frame.render_widget(title_block, layout_chunk);
    app.widget_rects.insert(Screen::Playlists, layout_chunk);
}
