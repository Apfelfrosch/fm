use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub fn process_keys(event: KeyEvent) -> bool {
    if let KeyEventKind::Press = event.kind {
        match event.code {
            KeyCode::Char('q') => true,
            _ => false,
        }
    } else {
        false
    }
}
