use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span, Text}, widgets::{Block, Borders, List, ListItem, Paragraph}, Frame};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create new JSON", 
        Style::default().fg(Color::Green)
    ));

    frame.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();
    for key in app.pairs.keys() {
        list_items.push(
            ListItem::new(Line::from(
                Span::styled(
                    format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
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