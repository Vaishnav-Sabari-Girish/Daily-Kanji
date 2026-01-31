use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
    match app.screen {
        CurrentScreen::Menu => render_menu(f),
        CurrentScreen::Quiz => render_quiz(f, app),
        CurrentScreen::Results => render_results(f, app),
        _ => {}
    }
}

fn render_menu(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(f.area());

    let content_area = centered_rect(60, 50, chunks[1]);

    let title = Paragraph::new(vec![
        Line::from("🇯🇵 Daily Kanji Quiz 🇯🇵".bold().yellow()),
        Line::from(""),
        Line::from("Select your JLPT Level to begin:"),
        Line::from(""),
        Line::from(vec!["[1]".cyan(), " JLPT N5".into()]),
        Line::from(vec!["[2]".cyan(), " JLPT N4".into()]),
        Line::from(vec!["[3]".cyan(), " JLPT N3".into()]),
        Line::from(""),
        Line::from(vec!["[Q]".red(), " Quit".into()]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Welcome "),
    );

    f.render_widget(title, content_area);
}

fn render_quiz(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Question Body
            Constraint::Length(3),  // Input Field
        ])
        .split(f.area());

    // --- Header Section ---
    let progress_ratio = (app.current_question_index as f64) / 10.0;
    let label = format!("Question {}/10", app.current_question_index + 1);
    
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(progress_ratio)
        .label(label);
    
    f.render_widget(gauge, chunks[0]);

    // --- Question Section ---
    if let Some(question) = app.questions.get(app.current_question_index) {
        let question_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(" Score: {} ", app.score));

        let text = vec![
            Line::from(""),
            Line::from("Read the following sentence:"),
            Line::from(""),
            Line::from(Span::styled(
                &question.sentence, 
                Style::default().add_modifier(Modifier::BOLD).fg(Color::White)
            )),
            Line::from(""),
            Line::from("--------------------------------"),
            Line::from(""),
            Line::from(vec![
                "Target Kanji: ".into(),
                Span::styled(&question.target_kanji, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from("Enter Reading (Kana): ".gray()),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(question_block);
        
        f.render_widget(paragraph, chunks[1]);
    }

    // --- Input Section ---
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Type Answer & Press Enter ");

    let input_text = Paragraph::new(app.user_input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(input_block);

    f.render_widget(input_text, chunks[2]);
    
    // Set cursor to the end of the input text
    f.set_cursor_position(
        (
            chunks[2].x + 1 + app.user_input.chars().count() as u16,
            chunks[2].y + 1,
        )
    );
}

fn render_results(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 40, f.area());

    let final_score_color = if app.score >= 7 { Color::Green } else { Color::Red };

    let text = vec![
        Line::from(""),
        Line::from("✨ Quiz Complete! ✨".bold()),
        Line::from(""),
        Line::from(vec![
            "Final Score: ".into(),
            // FIXED: Only show correct/total
            Span::styled(format!("{}/10", app.score), final_score_color), 
        ]),
        Line::from(""),
        Line::from("Generating PDF Report..."),
        Line::from(""),
        Line::from("Press [Enter] to exit.".gray()),
    ];

    let block = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(final_score_color))
                .title(" Results "),
        );

    f.render_widget(block, area);
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

    let vertical_chunk = popup_layout[1];

    let horiz_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical_chunk);

    horiz_layout[1]
}
