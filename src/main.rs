use std::{error::Error, io::stderr, time::Duration};

use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use window::Window;

mod keys;
mod window;

fn initialize_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(stderr(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

fn main() -> Result<(), Box<dyn Error>> {
    initialize_panic_hook();

    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    let mut window = Window::build_from_path_no_symlink(".")?;

    loop {
        terminal.draw(|frame| {
            window.render_to_frame(frame, frame.size());
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(key_event) = event::read()? {
                if crate::keys::process_keys(key_event, &mut window) {
                    break;
                }
            }
        }
    }

    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
