use indexmap::IndexMap;
use ratatui::crossterm::event::{self, KeyEvent, KeyEventKind};
use serde_json::Value;


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
    pub pairs: IndexMap<String, Value>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub value_input_error: Option<String>,
    pub should_quit: bool,
    pub should_print: bool,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: IndexMap::new(),
            currently_editing: None,
            current_screen: CurrentScreen::Main,
            value_input_error: None,
            should_quit: false,
            should_print: false,
        }
    }
    pub fn save_key_value(&mut self) {
        match self.parse_value_input() {
            Ok(json_value) => {
                self.pairs.insert(self.key_input.clone(), json_value);
                self.key_input = String::new();
                self.value_input = String::new();
                self.currently_editing = None;
                self.value_input_error = None;
            }
            Err(error) => {
                self.value_input_error = Some(format!("Invalid JSON: {}", error));
            }
        }
    }

    fn parse_value_input(&self) -> Result<Value, serde_json::Error> {
        let trimmed = self.value_input.trim();
        
        if trimmed.is_empty() {
            return Ok(Value::Null);
        }
        
        match serde_json::from_str(trimmed) {
            Ok(value) => Ok(value),
            Err(_) => {
                if !trimmed.starts_with(['{', '[', '"']) && 
                   !trimmed.parse::<f64>().is_ok() && 
                   !matches!(trimmed, "true" | "false" | "null") {
                    Ok(Value::String(trimmed.to_string()))
                } else {
                    serde_json::from_str(trimmed)
                }
            }
        }
    }

    pub fn format_json_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => format!("\"{}\"", s),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(_) | Value::Object(_) => {
                serde_json::to_string(value).unwrap_or_else(|_| "invalid".to_string())
            }
        }
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
                self.should_quit = true;
                self.should_print = true;
            },
            event::KeyCode::Char('n') => {
                self.should_quit = true;
                self.should_print = false;
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