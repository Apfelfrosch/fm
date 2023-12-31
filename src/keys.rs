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
            KeyCode::Char('h') => {
                if let Some(w) = app.selected_window_mut() {
                    let mut target_path =
                        w.path.canonicalize().unwrap().to_string_lossy().to_string();
                    w.stored_selection.insert(target_path.clone(), w.selected);
                    let old_dir_name = w.current_dir_name().clone();
                    target_path.push_str(std::path::MAIN_SEPARATOR_STR);
                    target_path.push_str("..");
                    let target_path = std::fs::canonicalize(&target_path).unwrap();
                    w.path = target_path;
                    w.refresh().unwrap();
                    w.selected = w
                        .entries
                        .iter()
                        .position(|(n, _)| n == &old_dir_name)
                        .unwrap_or(w.selected);
                }
            }
            KeyCode::Char('l') => {
                if let Some(w) = app.selected_window_mut() {
                    let mut target_path =
                        w.path.canonicalize().unwrap().to_string_lossy().to_string();
                    target_path.push_str(std::path::MAIN_SEPARATOR_STR);
                    let selected_entry = &w.entries[w.selected];
                    if selected_entry.1.is_dir() {
                        target_path.push_str(&w.entries[w.selected].0);
                        let target_path = std::fs::canonicalize(&target_path).unwrap();
                        w.path = target_path;
                        w.refresh().unwrap();
                    }
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
