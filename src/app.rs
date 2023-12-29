use std::path::PathBuf;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::window::{Window, WindowSplit};

pub struct App {
    pub splits: Vec<WindowSplit>,
    pub selected_split: usize,
    pub global_selection: GlobalSelection,
}

pub struct GlobalSelection {
    pub paths: Vec<PathBuf>,
}

impl App {
    pub fn new() -> Self {
        App {
            splits: Vec::new(),
            selected_split: 0,
            global_selection: GlobalSelection { paths: Vec::new() },
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

    pub fn render_to_frame(&self, frame: &mut Frame<'_>, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .direction(Direction::Horizontal)
            .split(area);

        if let Some(split) = self.selected_split() {
            split.render_to_frame(frame, layout[1]);
        }

        let selected_paths = &self.global_selection.paths;
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("Yanked paths ({})", selected_paths.len()));
        if selected_paths.is_empty() {
            frame.render_widget(
                Paragraph::new(Line::from("Nothing yanked yet.")).block(block),
                layout[0],
            );
        } else {
            let mut lines = Vec::new();
            for yanked_path in selected_paths {
                /*let canon = std::fs::canonicalize(yanked_path)
                .map(|pbuf| pbuf.to_string_lossy().to_string())
                .unwrap_or(String::from("Could not get canon path"));*/
                lines.push(Line::from(Span::from(yanked_path.to_str().unwrap())));
            }
            frame.render_widget(Paragraph::new(lines).block(block), layout[0]);
        }
    }
}
