use std::{collections::HashMap, error::Error, io::{self, stderr}};
use std::result::Result;
use ratatui::{crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}}, prelude::{Backend, CrosstermBackend}, Terminal};

use crate::{app::{App, CurrentScreen, CurrentlyEditing}, ui::ui};

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    println!("Hello, world!");

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode();
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture);
    terminal.show_cursor();

    if let Ok(do_print) = res {
        if do_print {
            app.print_json();
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
        }
    }
}