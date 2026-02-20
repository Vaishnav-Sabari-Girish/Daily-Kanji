use chrono::{Local, NaiveDate};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct AppState {
    last_run_date: NaiveDate,
}

pub fn should_run() -> bool {
    let proj_dirs = ProjectDirs::from("com", "user", "daily_kanji").expect("No home dir");
    let state_dir = proj_dirs.data_dir();
    let state_file = state_dir.join("state.json");

    if !state_dir.exists() {
        fs::create_dir_all(state_dir).unwrap();
    }

    let today = Local::now().date_naive();

    if let Ok(content) = fs::read_to_string(&state_file) {
        if let Ok(state) = serde_json::from_str::<AppState>(&content) {
            if state.last_run_date == today {
                return false; // Already ran today
            }
        }
    }

    // Update state to today immediately (or defer until quiz completion if preferred)
    let new_state = AppState { last_run_date: today };
    let _ = fs::write(state_file, serde_json::to_string(&new_state).unwrap());
    
    true
}
