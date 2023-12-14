// src/quiz/question.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub question_text: String,
    pub options: Vec<String>,
    pub correct_option: usize,
}
