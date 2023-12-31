use std::{
    cmp::Ordering,
    fs::{self, Metadata},
    io,
    path::PathBuf,
};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub type DirectoryEntry = (String, Metadata);

#[derive(Clone, Copy, Debug)]
pub enum WindowSplitSelection {
    First,
    Second,
}

impl WindowSplitSelection {
    pub fn opposite(&self) -> WindowSplitSelection {
        match &self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

pub struct WindowSplit {
    pub first: Window,
    pub selected: WindowSplitSelection,
    pub second: Option<Window>,
}

impl WindowSplit {
    pub fn single_window(w: Window) -> Self {
        WindowSplit {
            first: w,
            selected: WindowSplitSelection::First,
            second: None,
        }
    }

    pub fn two_windows(first: Window, second: Window) -> Self {
        WindowSplit {
            first,
            selected: WindowSplitSelection::First,
            second: Some(second),
        }
    }

    pub fn render_to_frame(&mut self, frame: &mut Frame<'_>, area: Rect) {
        if let Some(second_window) = &mut self.second {
            let layout = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .direction(ratatui::layout::Direction::Horizontal)
                .split(area);
            let first_is_selected = matches!(self.selected, WindowSplitSelection::First);
            self.first
                .render_to_frame(frame, first_is_selected, layout[0]);
            second_window.render_to_frame(frame, !first_is_selected, layout[1]);
        } else {
            self.first.render_to_frame(frame, true, area);
        }
    }

    pub fn selected(&self) -> &Window {
        if let Some(second_window) = &self.second {
            if let WindowSplitSelection::First = self.selected {
                &self.first
            } else {
                second_window
            }
        } else {
            &self.first
        }
    }

    pub fn selected_mut(&mut self) -> &mut Window {
        if let Some(second_window) = self.second.as_mut() {
            if let WindowSplitSelection::First = self.selected {
                &mut self.first
            } else {
                second_window
            }
        } else {
            &mut self.first
        }
    }
}

#[derive(Clone, Debug)]
pub struct Window {
    pub path: PathBuf,
    pub entries: Vec<DirectoryEntry>,
    pub selected: usize,
    pub sort_mode: SortMode,
    current_dir_name: String,
    scroll_y: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum SortMode {
    DirectoriesFirst,
    Ungrouped,
}

impl SortMode {
    pub fn next(&self) -> SortMode {
        match &self {
            SortMode::DirectoriesFirst => SortMode::Ungrouped,
            SortMode::Ungrouped => SortMode::DirectoriesFirst,
        }
    }
}

impl Window {
    pub fn build_from_path_no_symlink(path: PathBuf) -> io::Result<Window> {
        let mut w = Window {
            path,
            entries: Vec::new(),
            selected: 0,
            sort_mode: SortMode::Ungrouped,
            current_dir_name: String::from(""),
            scroll_y: 0,
        };
        w.refresh()?;
        Ok(w)
    }

    pub fn move_down(&mut self) -> bool {
        if self.selected + 1 >= self.entries.len() {
            false
        } else {
            self.selected += 1;
            true
        }
    }

    pub fn move_up(&mut self) -> bool {
        if self.selected == 0 {
            false
        } else {
            self.selected -= 1;
            true
        }
    }

    pub fn current_dir_name(&self) -> &String {
        &self.current_dir_name
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        let it = fs::read_dir(&self.path)?;
        let mut files = Vec::new();
        for dir_entry in it {
            let dir_entry = dir_entry?;
            let metadata = dir_entry.metadata()?;
            let entry = (
                dir_entry.file_name().to_string_lossy().to_string(),
                metadata,
            );
            files.push(entry);
        }
        let canon = self
            .path
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .to_string();
        self.current_dir_name = if canon.len() <= 1 {
            "/".to_string()
        } else {
            canon[canon.rfind('/').map(|idx| idx + 1).unwrap_or(0)..].to_string()
        };
        self.entries = files;
        if self.selected >= self.entries.len() {
            self.selected = self.entries.len().max(1) - 1;
        }
        self.sort_entries();
        self.scroll_y = 0;
        Ok(())
    }

    pub fn sort_entries(&mut self) {
        match self.sort_mode {
            SortMode::Ungrouped => self
                .entries
                .sort_by(|(n1, _), (n2, _)| n1.to_lowercase().cmp(&n2.to_lowercase())),
            SortMode::DirectoriesFirst => {
                self.entries.sort_by(|(n1, m1), (n2, m2)| {
                    if m1.is_dir() && !m2.is_dir() {
                        Ordering::Less
                    } else if !m1.is_dir() && m2.is_dir() {
                        Ordering::Greater
                    } else {
                        n1.to_lowercase().cmp(&n2.to_lowercase())
                    }
                });
            }
        }
    }

    pub fn render_to_frame(&mut self, frame: &mut Frame<'_>, is_selected: bool, area: Rect) {
        let last_seen =
            (self.scroll_y + (area.height - 3) as usize).min(self.entries.len().max(1) - 1);

        if self.selected > last_seen {
            self.scroll_y += self.selected - last_seen;
        }

        if self.selected < self.scroll_y {
            self.scroll_y -= self.scroll_y - self.selected;
        }

        let mut lines = Vec::new();
        for (idx, (name, metadata)) in self
            .entries
            .iter()
            .enumerate()
            .skip(self.scroll_y)
            .take(area.height as usize)
        {
            let color = if metadata.is_symlink() {
                Color::Blue
            } else if metadata.is_dir() {
                Color::LightBlue
            } else {
                Color::Gray
            };
            let mut span = Span::styled(name, Style::new().fg(color));
            let is_selected_entry = self.selected == idx;

            if is_selected_entry {
                span = span.add_modifier(Modifier::UNDERLINED);
                if is_selected {
                    span = span.bg(Color::Rgb(77, 77, 77));
                }
            }

            lines.push(Line::from(span));
        }
        frame.render_widget(
            Paragraph::new(lines).block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(self.current_dir_name.as_str())
                    .fg(if is_selected {
                        Color::Cyan
                    } else {
                        Color::Gray
                    }),
            ),
            area,
        );
    }
}
