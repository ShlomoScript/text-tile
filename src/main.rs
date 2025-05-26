mod app;
mod window;

use app::App;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use std::io::stdout;
use std::panic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = execute!(stdout(), DisableMouseCapture);
        default_hook(info);
    }));
    let mut app = App::new(terminal);
    app.run()
}
