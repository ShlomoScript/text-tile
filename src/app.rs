use crate::window::{Window, WindowManager};
use ratatui::layout::Rect;

pub struct App {
    pub wm: WindowManager,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        let wm = WindowManager::new();

        /*// this is sample window for testing purposes
        wm.add_window(Window {
            title: "Example Window".into(),
            area: Rect::new(10, 5, 40, 10),
            focused: true,
        });*/

        Self { wm, running: true }
    }

    pub fn add_sample_window(&mut self, area: Rect) {
        self.wm.add_window(Window {
            title: "Example window".into(),
            area,
            focused: true,
            held: false,
        });
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
