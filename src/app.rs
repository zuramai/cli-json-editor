use std::collections::HashMap;

use ratatui::crossterm::event::{self, KeyEvent, KeyEventKind};


pub enum CurrentScreen {
    Main,
    Editing,
    Exiting
}

pub enum CurrentlyEditing {
    Key,
    Value
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            pairs: HashMap::new(),
            currently_editing: None,
            value_input: String::new(),
            current_screen: CurrentScreen::Main,
        }
    }
    pub fn save_key_value(&mut self) {
        self.pairs.insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing  {
             match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
             }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        };
    }

    fn exit(&mut self) {

    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
    pub fn handle_editing_screen_input(&mut self, key: KeyEvent) {
        match key.code {
            event::KeyCode::Enter => {
                if let Some(editing) = &self.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            self.currently_editing = Some(CurrentlyEditing::Value)
                        },
                        CurrentlyEditing::Value => {
                            self.current_screen = CurrentScreen::Exiting;
                            self.save_key_value();
                        }
                    }
                }
                self.current_screen = CurrentScreen::Editing;
            },
            event::KeyCode::Backspace => {
                if let Some(editing) = &self.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            self.key_input.pop();
                        },
                        CurrentlyEditing::Value => {
                            self.value_input.pop();
                        }
                    }
                }
            }
            event::KeyCode::Esc => {
                self.current_screen = CurrentScreen::Main;
                self.currently_editing = None;
            },
            event::KeyCode::Tab => {
                self.toggle_editing();
            },
            event::KeyCode::Char(v) => {
                if let Some(editing) = &self.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            self.key_input.push(v)
                        },
                        CurrentlyEditing::Value => {
                            self.value_input.push(v)
                        }
                    }
                }
            }

            _ => {}
        }
    }

    pub fn handle_exiting_screen_input(&mut self, key: KeyEvent) {
        match key.code {
            event::KeyCode::Char('y') => {
                return self.exit()
            },
            event::KeyCode::Char('n') => {
                return 
            },
            _ => {}
        }
    }

    pub fn handle_main_screen_input(&mut self, key: KeyEvent) {
        match key.code {
            event::KeyCode::Char('e') => {
                self.current_screen = CurrentScreen::Editing;
                self.currently_editing = Some(CurrentlyEditing::Key)
            },
            event::KeyCode::Char('q') => {
                self.current_screen = CurrentScreen::Exiting;
            },
            _ => {}
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind == event::KeyEventKind::Release {
            return
        }
        match self.current_screen {
            self::CurrentScreen::Main => self.handle_main_screen_input(key),
            self::CurrentScreen::Exiting => self.handle_exiting_screen_input(key),
            self::CurrentScreen::Editing if key.kind == KeyEventKind::Press => self.handle_editing_screen_input(key),
            _ => {}
        }
    }
}