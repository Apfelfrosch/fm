use std::{
    cmp::Ordering,
    fs::{self, Metadata},
    io,
};

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub type DirectoryEntry = (String, Metadata);

#[derive(Clone, Debug)]
pub struct Window {
    path: String,
    entries: Vec<DirectoryEntry>,
    pub sort_mode: SortMode,
}

#[derive(Clone, Copy, Debug)]
pub enum SortMode {
    DirectoriesFirst,
    Ungrouped,
}

impl Window {
    pub fn build_from_path_no_symlink(path: impl Into<String>) -> io::Result<Window> {
        let mut w = Window {
            path: path.into(),
            entries: Vec::new(),
            sort_mode: SortMode::Ungrouped,
        };
        w.refresh()?;
        Ok(w)
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
        self.entries = files;
        self.sort_entries();
        Ok(())
    }

    pub fn sort_entries(&mut self) {
        match self.sort_mode {
            SortMode::Ungrouped => self.entries.sort_by(|(n1, _), (n2, _)| n1.cmp(n2)),
            SortMode::DirectoriesFirst => {
                self.entries.sort_by(|(n1, m1), (n2, m2)| {
                    if m1.is_dir() && !m2.is_dir() {
                        Ordering::Less
                    } else if !m1.is_dir() && m2.is_dir() {
                        Ordering::Greater
                    } else {
                        n1.cmp(n2)
                    }
                });
            }
        }
    }

    pub fn render_to_frame(&self, frame: &mut Frame<'_>, area: Rect) {
        let mut lines = Vec::new();
        for (name, metadata) in &self.entries {
            let color = if metadata.is_symlink() {
                Color::Blue
            } else if metadata.is_dir() {
                Color::LightBlue
            } else {
                Color::Gray
            };
            lines.push(Line::from(Span::styled(name, Style::new().fg(color))));
        }
        frame.render_widget(Paragraph::new(lines), area);
    }
}
