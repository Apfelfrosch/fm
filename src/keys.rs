use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::window::{SortMode, WindowSplit};

static mut MODE: bool = true;

pub fn process_keys(event: KeyEvent, split: &mut WindowSplit) -> bool {
    if let KeyEventKind::Press = event.kind {
        let mut should_quit = false;
        let w = split.selected_mut();
        match event.code {
            KeyCode::Char('q') => should_quit = true,
            KeyCode::Char('m') => unsafe {
                MODE = !MODE;
                w.sort_mode = if !MODE {
                    SortMode::DirectoriesFirst
                } else {
                    SortMode::Ungrouped
                };
                w.sort_entries();
            },
            KeyCode::Char('j') => {
                w.move_down();
            }
            KeyCode::Char('k') => {
                w.move_up();
            }
            _ => {}
        }
        should_quit
    } else {
        false
    }
}
