use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
    Frame, style::{Style, Modifier},
};

use crate::app::App;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App, layout_chunk: Rect) {
    let items: Vec<ListItem> = app.lines.iter().map(|line| ListItem::new(line.as_str())).collect();

    let liste = List::new(items)
        .block(Block::default().title("Search results").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    frame.render_widget(liste, layout_chunk);
}
