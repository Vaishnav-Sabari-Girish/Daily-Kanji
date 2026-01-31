mod app;
mod data;
mod report;
mod state;
mod ui;

use std::io;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::{App, CurrentScreen};
use crate::data::JlptLevel;

fn main() -> Result<()> {
    // 1. Daily Guard: Check if we should run today
    // UNCOMMENT THESE LINES TO ENABLE THE "ONCE PER DAY" LIMIT
    if !state::should_run() {
        println!("You have already practiced today. Come back tomorrow!");
        return Ok(());
    }

    // 2. Setup Terminal (Raw Mode)
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 3. Initialize App State
    let mut app = App::new();

    // 4. Run the Main TUI Loop
    let res = run_app(&mut terminal, &mut app);

    // 5. Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error during execution: {:?}", err);
    }

    // 6. Generate Report (if quiz was completed)
    if app.screen == CurrentScreen::Results || app.quiz_finished {
        println!("Generating daily PDF report...");
        match report::generate_report(&app) {
            Ok(_) => println!("Success! Report saved to ./daily_kanji_report.pdf"),
            Err(e) => eprintln!("Failed to generate report: {}", e),
        }
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.screen {
                CurrentScreen::Menu => match key.code {
                    KeyCode::Char('1') => app.start_quiz(JlptLevel::N5),
                    KeyCode::Char('2') => app.start_quiz(JlptLevel::N4),
                    KeyCode::Char('3') => app.start_quiz(JlptLevel::N3),
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                },
                CurrentScreen::Quiz => match key.code {
                    KeyCode::Enter => app.submit_answer(),
                    KeyCode::Char(c) => app.user_input.push(c),
                    KeyCode::Backspace => {
                        app.user_input.pop();
                    }
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                },
                CurrentScreen::Results => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => return Ok(()),
                    _ => {}
                },
                _ => {} // Ignore other events
            }
        }
    }
}
