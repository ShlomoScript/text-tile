use crate::window::{Window, WindowManager};
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind};
use crossterm::execute;
use ratatui::{DefaultTerminal, layout::Rect};
use std::io::stdout;

pub struct App {
    pub wm: WindowManager,
    pub terminal: DefaultTerminal,
    pub running: bool,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> Self {
        let wm = WindowManager::new();

        Self {
            wm,
            terminal,
            running: true,
        }
    }

    pub fn add_sample_window(&mut self, area: Rect) {
        self.wm.add_window(Window {
            title: "Example window".into(),
            area,
            focused: true,
            held: false,
            resizing: None,
        });
    }
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while self.running {
            self.terminal.draw(|f| self.wm.draw(f))?;
            if event::poll(std::time::Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) => match key.code {
                        KeyCode::Char('q') => self.quit(),
                        KeyCode::Char('a') => self.add_sample_window(Rect::new(10, 5, 40, 10)),
                        KeyCode::Tab => self.wm.next_window(),
                        _ => {}
                    },
                    Event::Mouse(mouse) => match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            self.wm.left_mouse_down(mouse.column, mouse.row)
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            self.wm.left_mouse_drag(mouse.column, mouse.row)
                        }
                        MouseEventKind::Up(MouseButton::Left) => self.wm.let_windows_go(),
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

    pub fn quit(&mut self) {
        self.running = false;
    }
}
