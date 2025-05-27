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
    pub resizing: Option<Resize>,
}

impl Window {
    pub fn draw(&self, f: &mut Frame) {
        let border_color = if self.focused {
            Color::Cyan
        } else {
            Color::White
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str())
            .border_style(Style::default().fg(border_color));
        let content = Paragraph::new(format!(
            "height: {}\nwidth: {}\nx: {}\ny: {}",
            self.area.height, self.area.width, self.area.x, self.area.y
        ))
        .block(block);
        f.render_widget(content, self.area);
    }
}
#[derive(PartialEq, Debug)]
pub enum Resize {
    Left,
    Right,
    Down,
    DownLeft,
    DownRight,
}

pub struct WindowManager {
    pub windows: Vec<Window>,
    pub current_offset: Option<u16>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            current_offset: None,
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
    pub fn left_mouse_down(&mut self, x: u16, y: u16) {
        if let Some(index) = self.get_window_at_cords(x, y) {
            self.focus_window(index);
            let window = self.windows.last_mut().unwrap();
            if y == window.area.y {
                window.held = true;
                self.current_offset = Some(x - window.area.x);
            } else if y == window.area.y + window.area.height - 1 {
                if x == window.area.x {
                    window.resizing = Some(Resize::DownLeft);
                } else if x == window.area.x + window.area.width - 1 {
                    window.resizing = Some(Resize::DownRight);
                } else {
                    window.resizing = Some(Resize::Down);
                }
            } else if x == window.area.x {
                window.resizing = Some(Resize::Left);
            } else if x == window.area.x + window.area.width - 1 {
                window.resizing = Some(Resize::Right);
            }
        }
    }
    pub fn left_mouse_drag(&mut self, x: u16, y: u16) {
        let offset = self.current_offset.unwrap_or(0);
        let window = self.windows.last_mut().unwrap();
        if window.held {
            window.area.x = x.saturating_sub(offset);
            window.area.y = y;
        } else if let Some(resize) = &window.resizing {
            match resize {
                Resize::Left => {
                    let width = (window.area.x + window.area.width).saturating_sub(x).max(2);
                    if window.area.width > 2 || window.area.x > x {
                        window.area.x = x;
                    }
                    window.area.width = width;
                }
                Resize::Right => {
                    let width = (x.saturating_sub(window.area.x) + 1).max(2);
                    window.area.width = width;
                }
                Resize::Down => {
                    let height = (y.saturating_sub(window.area.y) + 1).max(2);
                    window.area.height = height;
                }
                Resize::DownLeft => {
                    let width = (window.area.x + window.area.width).saturating_sub(x).max(2);
                    let height = (y.saturating_sub(window.area.y) + 1).max(2);
                    if window.area.width > 2 || window.area.x > x {
                        window.area.x = x;
                    }
                    window.area.width = width;
                    window.area.height = height;
                }
                Resize::DownRight => {
                    let width = (x.saturating_sub(window.area.x) + 1).max(2);
                    let height = (y.saturating_sub(window.area.y) + 1).max(2);
                    window.area.width = width;
                    window.area.height = height;
                }
            }
        }
    }

    pub fn let_windows_go(&mut self) {
        self.windows.iter_mut().for_each(|window| {
            window.held = false;
            window.resizing = None;
        });
        self.current_offset = None;
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
            window.draw(f);
        }
    }
}
