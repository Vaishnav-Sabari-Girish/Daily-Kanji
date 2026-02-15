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
    pub user_answers: Vec<(String, bool)>, // (Answer, IsCorrect)
    pub quiz_finished: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: CurrentScreen::Menu,
            selected_level: None,
            questions: vec![],
            current_question_index: 0,
            user_input: String::new(),
            score: 0,
            user_answers: vec![],
            quiz_finished: false,
        }
    }

    pub fn start_quiz(&mut self, level: JlptLevel) {
        self.selected_level = Some(level);
        self.questions = load_questions(level);
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
        // Trim whitespace for simple validation
        let is_correct = self.user_input.trim() == current_q.correct_reading;

        if is_correct {
            self.score += 1;
        } 
        // No negative marking for wrong answers

        self.user_answers.push((self.user_input.clone(), is_correct));
        self.user_input.clear();

        // Check if we reached the end (15 questions = index 14)
        if self.current_question_index >= 14 {
            self.quiz_finished = true;
            self.screen = CurrentScreen::Results;
        } else {
            self.current_question_index += 1;
        }
    }
}
