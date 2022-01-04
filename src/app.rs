use std::{io, cmp::{max, min}};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use tui::{backend::Backend, Terminal};

use crate::{ui, youtubeclient::YoutubeClient, utils};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub input_cursor_position: usize,
    pub shoud_run: bool,

    pub yt_client: Box<YoutubeClient>,
    pub lines: Vec<String>,
}

impl App {
    pub fn new(yt_client: Box<YoutubeClient>) -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            input_cursor_position: 0,
            shoud_run: true,
            yt_client,
            lines: Vec::new(),
        }
    }

    pub async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| ui::draw_main_layout(frame, self))?;
            self.update_events().await?;

            if !self.shoud_run {
                return Ok(());
            }
        }
    }

    pub async fn update_events(&mut self) -> Result<(), io::Error> {
        if let Event::Key(key) = event::read()? {
            // check if it's not ctrl-c
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                self.shoud_run = false;
                return Ok(());
            } 

            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('s') => {
                        self.input_mode = InputMode::Editing;
                        self.input_cursor_position = self.input.len();
                    }
                    KeyCode::Char('q') => {
                        self.shoud_run = false;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        self.lines.clear();

                        let videos = self.yt_client.search_music(&self.input, 10).await;
                        videos.iter().for_each(|video| self.lines.push(utils::display_video_line(video.clone())));
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
                    KeyCode::Esc => {
                        self.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
}
