use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
#[derive(PartialEq)]
pub struct Window {
    pub title: String,
    pub area: Rect,
    pub focused: bool,
    pub held: bool,
}

pub struct WindowManager {
    pub windows: Vec<Window>,
    pub current_offset: Option<u16>,
    pub holding_index: Option<usize>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            current_offset: None,
            holding_index: None,
        }
    }

    pub fn add_window(&mut self, mut window: Window) {
        for w in &mut self.windows {
            w.focused = false;
        }

        window.focused = true;
        self.windows.push(window);
    }

    fn get_window_at_cords(&mut self, x: u16, y: u16) -> Option<usize> {
        self.windows
            .iter_mut()
            .enumerate()
            .rev()
            .find(|(_, window)| window.area.contains(Position { x, y }))
            .map(|(index, _)| index)
    }

    pub fn hold_window(&mut self, x: u16, y: u16) {
        if let Some(index) = self.get_window_at_cords(x, y) {
            let window = self.windows.get_mut(index).unwrap();
            if y == window.area.y {
                window.held = true;
                self.current_offset = Some(x - window.area.x);
                self.holding_index = Some(index);
            }
        }
    }
    pub fn let_windows_go(&mut self) {
        self.windows
            .iter_mut()
            .for_each(|window| window.held = false);
        self.current_offset = None;
        self.holding_index = None;
    }
    pub fn move_window(&mut self, x: u16, y: u16) {
        let offset = self.current_offset.unwrap_or(0);
        if let Some(index) = self.holding_index {
            if let Some(window) = self.windows.get_mut(index) {
                if window.held {
                    window.area.x = x - offset;
                    window.area.y = y;
                }
            }
        }
    }
    pub fn focus_window(&mut self, index: usize) {
        self.windows
            .iter_mut()
            .for_each(|window| window.focused = false);
        let mut window = self.windows.remove(index);
        window.focused = true;
        self.windows.push(window);
    }
    pub fn next_window(&mut self) {
        self.focus_window(0);
    }

    pub fn draw(&self, f: &mut Frame) {
        for window in &self.windows {
            let border_color = if window.focused {
                Color::Cyan
            } else {
                Color::White
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .title(window.title.as_str())
                .border_style(Style::default().fg(border_color));
            let content = Paragraph::new("Window content").block(block);
            f.render_widget(content, window.area);
        }
    }
}
