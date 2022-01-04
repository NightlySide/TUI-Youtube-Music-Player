use std::{io, cmp::{max, min}, collections::HashMap, time::{Instant, Duration}};

use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseEventKind, MouseButton};
use google_youtube3::api::Video;
use tui::{backend::Backend, Terminal, layout::Rect};

use crate::{ui::{self, searchresults::StatefulList}, youtubeclient::YoutubeClient, utils};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Screen {
    SearchBar,
    Playlists,
    SearchResults,
    PlayBar,
    None,
}

pub struct App {
    pub input: String,
    pub input_cursor_position: usize,
    pub shoud_run: bool,

    pub focused_screen: Screen,
    pub widget_rects: HashMap<Screen, Rect>,
    pub mouse_down: bool,

    pub yt_client: Box<YoutubeClient>,
    pub search_results: StatefulList<(String, Video)>,
}

impl App {
    pub fn new(yt_client: Box<YoutubeClient>) -> Self {
        Self {
            input: String::new(),
            input_cursor_position: 0,
            shoud_run: true,
            focused_screen: Screen::SearchBar,
            widget_rects: HashMap::new(),
            mouse_down: false,
            yt_client,
            search_results: StatefulList::default(),
        }
    }

    pub async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>, tick_rate: Duration) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            self.widget_rects.clear();
            terminal.draw(|frame| ui::draw_main_layout(frame, self))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                let events = event::read()?;
                self.get_key_events(&events).await?;
                self.get_mouse_events(&events)?;
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }

            if !self.shoud_run {
                return Ok(());
            }
        }
    }

    pub fn get_mouse_events(&mut self, events: &Event) -> Result<(), io::Error> {
        if let Event::Mouse(ev) = events {
            if let MouseEventKind::Down(btn) = ev.kind {
                if !self.mouse_down && btn == MouseButton::Left {
                    self.mouse_down = true;
                    if let Some(scr) = self.get_clicked_screen(ev.row, ev.column) {
                        self.focused_screen = scr;
                    }
                }
            } else if let MouseEventKind::Up(btn) = ev.kind {
                if self.mouse_down && btn == MouseButton::Left {
                    self.mouse_down = false;
                }
            }
        }

        Ok(())
    }

    pub async fn get_key_events(&mut self, events: &Event) -> Result<(), io::Error> {
        if let Event::Key(key) = events {
            // check if it's not ctrl-c
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                self.shoud_run = false;
                return Ok(());
            } 

            // unselect everything if escape is pressed
            if key.code == KeyCode::Esc {
                self.focused_screen = Screen::None;
                return Ok(());
            }

            match self.focused_screen {
                Screen::None => match key.code {
                    KeyCode::Char('s') => {
                        self.focused_screen = Screen::SearchBar;
                        self.input_cursor_position = self.input.len();
                    }
                    KeyCode::Char('q') => {
                        self.shoud_run = false;
                    }
                    _ => {}
                },
                Screen::SearchBar => match key.code {
                    KeyCode::Enter => {
                        self.search_results.clear();

                        let videos = self.yt_client.search_music(&self.input, 10).await;
                        videos.iter().for_each(|video| 
                            self.search_results.items.push((utils::display_video_line(video.clone()), video.clone()))
                        );
                    }
                    KeyCode::Char(c) => {
                        self.input.insert(self.input_cursor_position, c);
                        self.input_cursor_position += 1;
                    }
                    KeyCode::Backspace => {
                        if self.input_cursor_position > 0 {
                            self.input.remove(self.input_cursor_position - 1);
                            self.input_cursor_position -= 1;
                        }
                    }
                    KeyCode::Left => {
                        self.input_cursor_position = max(self.input_cursor_position - 1, 0);
                    }
                    KeyCode::Right => {
                        self.input_cursor_position = min(self.input_cursor_position + 1, self.input.len());
                    }
                    _ => {}
                },
                Screen::SearchResults => match key.code {
                    KeyCode::Up => self.search_results.previous(),
                    KeyCode::Down => self.search_results.next(),
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn get_clicked_screen(&self, row: u16, col: u16) -> Option<Screen> {
        for (k, v) in self.widget_rects.iter() {
            if row >= v.top() && row <= v.bottom() {
                if col >= v.left() && col <= v.right() {
                    return Some(k.clone());
                }
            }
        }

        None
    }
}
