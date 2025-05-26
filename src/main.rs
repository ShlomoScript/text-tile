mod app;
mod window;

use app::App;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind,
};
use crossterm::execute;
use ratatui::prelude::*;
use std::io::stdout;
use std::panic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = execute!(stdout(), DisableMouseCapture);
        default_hook(info);
    }));
    let mut app = App::new();

    while app.running {
        terminal.draw(|f| {
            let _size = f.area();
            app.wm.draw(f);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Char('a') => app.add_sample_window(Rect::new(10, 5, 40, 10)),
                    KeyCode::Tab => app.wm.next_window(),
                    _ => {}
                },
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        app.wm.hold_window(mouse.column, mouse.row)
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        app.wm.move_window(mouse.column, mouse.row)
                    }
                    MouseEventKind::Up(MouseButton::Left) => app.wm.let_windows_go(),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    execute!(stdout(), DisableMouseCapture)?;
    ratatui::restore();
    Ok(())
}
