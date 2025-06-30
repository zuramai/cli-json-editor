use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span, Text}, widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap}, Frame};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &App) {
    // init layout
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // init title block
    let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create new JSON", 
        Style::default().fg(Color::Green)
    ))
    .block(title_block); // render paragraph in the title block

    frame.render_widget(title, chunks[0]);

    // render json pairs as ListItem
    let mut list_items = Vec::<ListItem>::new();
    for (key, value) in &app.pairs {
        list_items.push(
            ListItem::new(Line::from(
                Span::styled(
                    format!("{: <25} : {}", key, app.format_json_value(value)),
                    Style::default().fg(Color::Yellow)
                )
            ))
        );
    }
    let list = List::new(list_items);
    frame.render_widget(list, chunks[1]);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
            CurrentScreen::Exiting => Span::styled("Exiting Mode", Style::default().fg(Color::LightRed)),
        },
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => Span::styled("Editing JSON key", Style::default().fg(Color::Green)),
                    CurrentlyEditing::Value => Span::styled("Editing JSON value", Style::default().fg(Color::LightGreen)),
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        }
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Exiting => Span::styled("(q) to quit / (e) to make new pair", Style::default().fg(Color::Red)),
            CurrentScreen::Editing => Span::styled("(Esc) to quit / (Tab) to switch boxes/enter to complete", Style::default().fg(Color::Red)),
            CurrentScreen::Main => Span::styled("(q) to quit / (e) to make new pair", Style::default().fg(Color::Red)),
        }
    };
    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    // editing popup
    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        let area = centered_rect(60, 35, frame.area()); // Increased height for error messages
        frame.render_widget(popup_block, area);

        // Create layout with space for error message
        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Input fields
                Constraint::Length(2), // Error message
                Constraint::Min(1),    // Instructions
            ])
            .split(area);

        // Input fields layout
        let input_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(popup_chunks[0]);
    
        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default()
            .title("Value (JSON)")
            .borders(Borders::ALL);
        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
    
        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => value_block = value_block.style(active_style),
        }
        
        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        frame.render_widget(key_text, input_chunks[0]);

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        frame.render_widget(value_text, input_chunks[1]);

        // Error message
        if let Some(error) = &app.value_input_error {
            let error_block = Block::default()
                .title("Error")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red));
            let error_text = Paragraph::new(error.clone())
                .block(error_block)
                .style(Style::default().fg(Color::Red));
            frame.render_widget(error_text, popup_chunks[1]);
        }

        // Instructions
        let instructions = vec![
            "Examples:",
            "• String: hello world  (or \"hello world\")",
            "• Number: 42",
            "• Boolean: true",
            "• Array: [1, 2, 3]",
            "• Object: {\"name\": \"John\", \"age\": 30}",
            "• Null: null",
        ];
        
        let instructions_text = instructions
            .iter()
            .enumerate()
            .map(|(i, &line)| {
                if i == 0 {
                    Line::from(Span::styled(line, Style::default().fg(Color::Cyan)))
                } else {
                    Line::from(Span::styled(line, Style::default().fg(Color::Gray)))
                }
            })
            .collect::<Vec<_>>();

        let instructions_paragraph = Paragraph::new(instructions_text)
            .block(Block::default().borders(Borders::ALL).title("JSON Examples"));
        frame.render_widget(instructions_paragraph, popup_chunks[2]);
    }

    // exit screen
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area());
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffers as json? (y/n)",
            Style::default().fg(Color::Red)
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap {
                trim: false
            });
        let area =  centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }

    
}
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
}