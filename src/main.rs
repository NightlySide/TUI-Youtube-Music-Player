use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use youtubeclient::YoutubeClient;

mod app;
mod ui;
mod utils;
mod youtubeclient;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // creating youtube client
    let yt_client = Box::new(YoutubeClient::new().await);

    start_app(yt_client).await
}

async fn start_app(yt_client: Box<YoutubeClient>) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // running the app
    let mut app = app::App::new(yt_client);
    let run_res = app.run_app(&mut terminal).await;

    // restoring the context
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = run_res {
        println!("{:?}", err);
    }
    Ok(())
}