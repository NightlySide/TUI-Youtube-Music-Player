use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, style::{Style, Modifier, Color},
};

use crate::app::{App, Screen};

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> Default for StatefulList<T> {
    fn default() -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
        }
    }
}

impl<T> StatefulList<T> {
    pub fn clear(&mut self) {
        self.items.clear();
        self.state = ListState::default();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn _unselect(&mut self) {
        self.state.select(None);
    }
}


pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let items: Vec<ListItem> = app.search_results.items
        .iter()
        .map(|(line, _)| ListItem::new(line.as_str())).collect();

    let liste = List::new(items)
        .style(match app.focused_screen {
            Screen::SearchResults => Style::default().fg(Color::LightRed),
            _ => Style::default(),
        })
        .block(Block::default().title("Search results").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC).fg(Color::LightYellow))
        .highlight_symbol(">");

    frame.render_stateful_widget(liste, layout_chunk, &mut app.search_results.state);
    app.widget_rects.insert(Screen::SearchResults, layout_chunk);
}
