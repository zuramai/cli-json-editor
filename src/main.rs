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
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                app::CurrentScreen::Main => match key.code {
                    event::KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                    },
                    event::KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    },
                    _ => {}
                },
                app::CurrentScreen::Exiting => match key.code {
                    event::KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Editing;
                    },
                    event::KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    },
                    _ => {}
                },
                app::CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    event::KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.currently_editing = Some(CurrentlyEditing::Value)
                                },
                                CurrentlyEditing::Value => {
                                    app.save_key_value();
                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                        app.current_screen = CurrentScreen::Editing;
                    },
                    event::KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.pop();
                                },
                                CurrentlyEditing::Value => {
                                    app.value_input.pop();
                                }
                            }
                        }
                    }
                    event::KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    },
                    event::KeyCode::Tab => {
                        app.toggle_editing();
                    },
                    event::KeyCode::Char(v) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.push(v)
                                },
                                CurrentlyEditing::Value => {
                                    app.value_input.push(v)
                                }
                            }
                        }
                    }

                    _ => {}
                },
                _ => {}
            }
        }
    }
}