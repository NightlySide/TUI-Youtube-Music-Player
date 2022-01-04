use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

mod playbar;
mod playlists;
mod searchbar;
mod searchresults;

pub fn draw_main_layout<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(6),
            ]
            .as_ref(),
        )
        .margin(0)
        .split(frame.size());

    // draw searchbar
    searchbar::draw(frame, app, parent_layout[0]);

    // draw content
    draw_content(frame, app, parent_layout[1]);

    // draw playing
    playbar::draw(frame, app, parent_layout[2]);
}

pub fn draw_content<B: Backend>(frame: &mut Frame<B>, app: &App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    // drawing playlists
    playlists::draw(frame, app, chunks[0]);

    // drawing search results
    searchresults::draw(frame, app, chunks[1]);
}
