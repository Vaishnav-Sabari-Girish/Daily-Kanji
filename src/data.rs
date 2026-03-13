use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
// thread_rng is removed, we will call rand::rng() directly

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum JlptLevel {
    N5,
    N4,
    N3,
}

#[derive(Debug, Serialize, Deserialize)]
struct QuestionSet {
    level: String,
    source: String,
    questions: Vec<Question>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    #[allow(dead_code)]
    pub id: u32,
    pub sentence: String,
    pub target_kanji: String,
    #[serde(rename = "expected_reading")]
    pub correct_reading: String,
    pub full_kana: String
}

pub fn load_questions(level: JlptLevel, limit: usize) -> Vec<Question> {
    let content = match level {
        JlptLevel::N5 => include_str!("../assets/n5.json"),
        JlptLevel::N4 => include_str!("../assets/n4.json"),
        JlptLevel::N3 => include_str!("../assets/n3.json"),
    };

    let mut question_set: QuestionSet = match serde_json::from_str(content) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing JSON for {:?}: {}", level, e);
            return vec![];
        }
    };

    // Use the new rand::rng() instead of the deprecated thread_rng()
    let mut rng = rand::rng();
    
    question_set.questions.shuffle(&mut rng);
    question_set.questions.into_iter().take(limit).collect()
}
