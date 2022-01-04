use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
//use unicode_width::UnicodeWidthStr;

use crate::app::{App, Screen};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let msg = match app.focused_screen {
        Screen::SearchBar => {
            let mut msg = String::from("> ");
            msg.push_str(&app.input);
            msg
        }
        _ => app.input.clone(),
    };

    let input = Paragraph::new(msg)
        .style(match app.focused_screen {
            Screen::SearchBar => Style::default().fg(Color::LightRed),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"));

    frame.render_widget(input, layout_chunk);
    app.widget_rects.insert(Screen::SearchBar, layout_chunk);

    // showing the cursor
    match app.focused_screen {
        Screen::SearchBar => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            frame.set_cursor(
                // Put cursor past the end of the input text
                layout_chunk.x + app.input_cursor_position as u16 + 3,
                // Move one line down, from the border to the input line
                layout_chunk.y + 1,
            )
        }
        _ =>
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        {}
    }
}
