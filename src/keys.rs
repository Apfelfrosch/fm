use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::App;

pub fn process_keys(event: KeyEvent, app: &mut App) -> bool {
    if let KeyEventKind::Press = event.kind {
        let mut should_quit = false;
        match event.code {
            KeyCode::Tab => {
                if let Some(split) = app.selected_split_mut() {
                    if split.second.is_none() {
                        split.second = Some(split.first.clone());
                    }
                    split.selected = split.selected.opposite();
                }
            }
            KeyCode::Char('q') => should_quit = true,
            KeyCode::Char('m') => {
                if let Some(w) = app.selected_window_mut() {
                    w.sort_mode = w.sort_mode.next();
                    w.sort_entries();
                }
            }
            KeyCode::Char('j') => {
                if let Some(w) = app.selected_window_mut() {
                    w.move_down();
                }
            }
            KeyCode::Char('k') => {
                if let Some(w) = app.selected_window_mut() {
                    w.move_up();
                }
            }
            KeyCode::Char('y') => {
                if let Some(w) = app.selected_window() {
                    let mut target_path =
                        w.path.canonicalize().unwrap().to_string_lossy().to_string();
                    target_path.push_str(std::path::MAIN_SEPARATOR_STR);
                    target_path.push_str(&w.entries[w.selected].0);
                    clipfile::put_path_into_clipboard(&target_path).expect("Could not copy");
                }
            }
            _ => {}
        }
        should_quit
    } else {
        false
    }
}
