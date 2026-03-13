use crate::data::{load_questions, JlptLevel, Question};

#[derive(PartialEq)]
pub enum CurrentScreen {
    Menu,
    Quiz,
    Results,
}

pub struct App {
    pub screen: CurrentScreen,
    pub selected_level: Option<JlptLevel>,
    pub questions: Vec<Question>,
    pub current_question_index: usize,
    pub user_input: String,
    pub score: i32,
    pub user_answers: Vec<(String, bool)>,
    pub quiz_finished: bool,
    pub question_limit: usize, // New field to track the limit
}

impl App {
    pub fn new(limit: usize) -> Self {
        Self {
            screen: CurrentScreen::Menu,
            selected_level: None,
            questions: vec![],
            current_question_index: 0,
            user_input: String::new(),
            score: 0,
            user_answers: vec![],
            quiz_finished: false,
            question_limit: limit,
        }
    }

    pub fn start_quiz(&mut self, level: JlptLevel) {
        self.selected_level = Some(level);
        // Pass the limit to the data loader
        self.questions = load_questions(level, self.question_limit);
        self.current_question_index = 0;
        self.score = 0;
        self.user_answers.clear();
        self.user_input.clear();
        self.quiz_finished = false;
        self.screen = CurrentScreen::Quiz;
    }

    pub fn submit_answer(&mut self) {
        if self.questions.is_empty() {
            return;
        }

        let current_q = &self.questions[self.current_question_index];

        // String sanitization
        let expected_clean = current_q.full_kana.replace("。", " ").replace("、", " ").replace("　", " ");
        let user_clean = self.user_input.trim().replace("。", " ").replace("、", " ").replace("　", " ");

        let is_correct = user_clean == expected_clean;

        if is_correct {
            self.score += 1;
        }

        self.user_answers.push((self.user_input.clone(), is_correct));
        self.user_input.clear();

        if self.current_question_index >= self.question_limit.saturating_sub(1) {
            self.quiz_finished = true;
            self.screen = CurrentScreen::Results;
        } else {
            self.current_question_index += 1;
        }
    }
}
