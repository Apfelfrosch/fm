use ratatui::{layout::Rect, Frame};

use crate::window::{Window, WindowSplit};

pub struct App {
    pub splits: Vec<WindowSplit>,
    pub selected_split: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            splits: Vec::new(),
            selected_split: 0,
        }
    }

    pub fn selected_split(&self) -> Option<&WindowSplit> {
        self.splits.get(self.selected_split)
    }

    pub fn selected_split_mut(&mut self) -> Option<&mut WindowSplit> {
        self.splits.get_mut(self.selected_split)
    }

    pub fn selected_window(&self) -> Option<&Window> {
        self.selected_split().map(|split| split.selected())
    }

    pub fn selected_window_mut(&mut self) -> Option<&mut Window> {
        self.selected_split_mut().map(|split| split.selected_mut())
    }

    pub fn new_split_single(&mut self, window: Window) -> &mut WindowSplit {
        let split = WindowSplit::single_window(window);
        self.splits.push(split);
        let idx = self.splits.len() - 1;
        &mut self.splits[idx]
    }

    pub fn new_split_both(&mut self, first: Window, second: Window) -> &mut WindowSplit {
        let split = WindowSplit::two_windows(first, second);
        self.splits.push(split);
        let idx = self.splits.len() - 1;
        &mut self.splits[idx]
    }

    pub fn render_to_frame(&mut self, frame: &mut Frame<'_>, area: Rect) {
        if let Some(split) = self.selected_split_mut() {
            split.render_to_frame(frame, area);
        }
    }
}
