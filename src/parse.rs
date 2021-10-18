use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Question {
    pub path: String,
    pub question: String,
    pub answer: Vec<Question>,
    pub text: Option<String>,
    pub option: Option<String>,
    pub end: Option<bool>,
}

impl Question {
    pub fn new(
        path: &str,
        question: &str,
        answer: Vec<Question>,
        text: Option<&str>,
        option: Option<&str>,
        end: Option<bool>,
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
            path: path.to_string(),
            question: question.to_string(),
            answer,
            text: new_text,
            option: new_option,
            end,
        }
    }

    pub fn parse_json(raw_data: String) -> Option<Question> {
        let data: Value = serde_json::from_str(&raw_data).ok()?;

        let answers = Question::parse_answers(data["answer"].clone(), "0");

        Some(Question::new(
            "0",
            data["question"].as_str().unwrap(),
            answers,
            data["text"].as_str(),
            None,
            data["end"].as_bool(),
        ))
    }

    fn parse_answers(data: Value, path: &str) -> Vec<Question> {
        let mut questions = Vec::new();
        let answer = match data.as_array() {
            Some(a) => a,
            None => return Vec::new(),
        };

        let mut index = 0;
        for i in answer {
            let new_path = &format!("{}-{}", path, index);
            questions.push(Question::new(
                new_path,
                i["question"].as_str().unwrap_or(""),
                Question::parse_answers(i["answer"].clone(), new_path),
                i["text"].as_str(),
                i["option"].as_str(),
                i["end"].as_bool(),
            ));
            index += 1;
        }

        questions
    }

    pub fn jsonify(self) -> String {
        self.jsonify_in(0)
    }

    fn jsonify_in(self, depth: usize) -> String {
        if depth > 1 {
            return "".to_string();
        }

        let mut text = String::new();
        if self.text.is_some() {
            text = format!(r#""text":"{}","#, self.text.unwrap())
        }

        let mut option = String::new();
        if self.option.is_some() {
            option = format!(r#""option":"{}","#, self.option.unwrap())
        }

        let mut end = String::new();
        if self.end.is_some() {
            end = format!(r#""end":{},"#, self.end.unwrap())
        }

        let mut answers = String::new();
        for i in self.answer {
            let json = i.jsonify_in(depth + 1);
            answers.push_str(&format!(
                r#"{}{}"#,
                json,
                if json.is_empty() { "" } else { "," }
            ));
        }
        if !answers.is_empty() {
            answers = answers[0..answers.len() - 1].to_string();
            answers = format!(r#","answer":[{}]"#, answers);
        }

        let working = &format!(
            r#""path":"{}",{}{}{}"question":"{}"{}"#,
            self.path, text, option, end, self.question, answers
        );

        format!(r#"{{{}}}"#, working.replace('\n', "\\n"))
    }
}
