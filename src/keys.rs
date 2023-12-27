use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::window::{SortMode, Window};

static mut MODE: bool = true;

pub fn process_keys(event: KeyEvent, w: &mut Window) -> bool {
    if let KeyEventKind::Press = event.kind {
        match event.code {
            KeyCode::Char('q') => true,
            KeyCode::Char('m') => unsafe {
                MODE = !MODE;
                w.sort_mode = if !MODE {
                    SortMode::DirectoriesFirst
                } else {
                    SortMode::Ungrouped
                };
                w.sort_entries();
                false
            },
            _ => false,
        }
    } else {
        false
    }
}
