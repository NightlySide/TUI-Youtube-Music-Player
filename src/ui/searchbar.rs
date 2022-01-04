use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
//use unicode_width::UnicodeWidthStr;

use crate::app::{App, InputMode};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App, layout_chunk: Rect) {
    let msg = match app.input_mode {
        InputMode::Normal => app.input.clone(),
        InputMode::Editing => {
            let mut msg = String::from("> ");
            msg.push_str(&app.input);
            msg
        }
    };

    let input = Paragraph::new(msg)
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::LightRed),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"));

    frame.render_widget(input, layout_chunk);

    // showing the cursor
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            frame.set_cursor(
                // Put cursor past the end of the input text
                layout_chunk.x + app.input_cursor_position as u16 + 3,
                // Move one line down, from the border to the input line
                layout_chunk.y + 1,
            )
        }
    }
}
