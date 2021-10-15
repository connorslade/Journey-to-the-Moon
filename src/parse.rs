use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Question {
    pub question: String,
    pub answer: Vec<Question>,
    pub text: Option<String>,
    pub option: Option<String>,
}

impl Question {
    pub fn new(
        question: &str,
        answer: Vec<Question>,
        text: Option<&str>,
        option: Option<&str>,
    ) -> Question {
        let mut new_text: Option<String> = None;
        let mut new_option: Option<String> = None;
        if text.is_some() {
            new_text = Some(text.unwrap_or("").to_string());
        }
        if option.is_some() {
            new_option = Some(option.unwrap_or("").to_string());
        }

        Question {
            question: question.to_string(),
            answer,
            text: new_text,
            option: new_option,
        }
    }

    pub fn parse_json(raw_data: String) -> Option<Question> {
        let data: Value = serde_json::from_str(&raw_data).ok()?;

        let answers = Question::parse_answers(data["answer"].clone());

        Some(Question::new(
            data["question"].as_str().unwrap(),
            answers,
            data["text"].as_str(),
            None,
        ))
    }

    fn parse_answers(data: Value) -> Vec<Question> {
        let mut questions = Vec::new();
        let answer = match data.as_array() {
            Some(a) => a,
            None => return Vec::new(),
        };

        for i in answer {
            questions.push(Question::new(
                i["question"].as_str().unwrap_or(""),
                Question::parse_answers(i["answer"].clone()),
                i["text"].as_str(),
                i["option"].as_str(),
            ));
        }

        questions
    }
}
