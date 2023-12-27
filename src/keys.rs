use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::window::WindowSplit;

pub fn process_keys(event: KeyEvent, split: &mut WindowSplit) -> bool {
    if let KeyEventKind::Press = event.kind {
        let mut should_quit = false;
        let w = split.selected_mut();
        match event.code {
            KeyCode::Tab => {
                if split.second.is_none() {
                    split.second = Some(split.first.clone());
                }
                split.selected = split.selected.opposite();
            }
            KeyCode::Char('q') => should_quit = true,
            KeyCode::Char('m') => {
                w.sort_mode = w.sort_mode.next();
                w.sort_entries();
            }
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
