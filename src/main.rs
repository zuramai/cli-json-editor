use std::{error::Error, io};
use ratatui::{crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}}, prelude::{Backend, CrosstermBackend}, Terminal};

use crate::{app::App, ui::ui};

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture);
    let _ = terminal.show_cursor();

    if let Ok(do_print) = res {
        if do_print {
            let _ = app.print_json();
        }
    } else if let Err(e) = res {
        println!("{e:?}");
    }
    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        
        if let Event::Key(key) = event::read()? {
            app.handle_key_event(key);
            
            // Check if we should exit
            if app.should_quit {
                return Ok(app.should_print);
            }
        }
    }
}