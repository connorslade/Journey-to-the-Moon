use crate::common;
use crate::parse::Question;

pub fn local_hame(data: Question) {
    println!("[*] Starting Game\n");
    let mut question = data;
    loop {
        let mut to_print = String::new();
        if question.clone().text.is_some() {
            to_print.push_str(&question.clone().text.unwrap());
            to_print.push_str("\n\n");
        }
        to_print.push_str(&question.question);
        to_print.push_str("\n\n");

        if question.end.is_some() {
            if question.end.unwrap() {
                println!("{}\nYou Win!", to_print);
                return;
            }
            println!("{}\nYou Loose...", to_print);
            return;
        }

        let mut index = 0;
        for i in question.clone().answer {
            index += 1;
            to_print.push_str(&format!(
                "{}) {}   ",
                index,
                i.option.unwrap_or_else(|| "".to_string())
            ))
        }

        println!("{}", to_print);
        let choice = input!("> ");
        if common::contains_any(&choice.to_lowercase(), &["exit", "quit", "go away"]) {
            return;
        }
        println!();

        let choice_i = match choice.parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                println!("Invalid Option!\n");
                continue;
            }
        };

        if choice_i > question.answer.len() || choice_i == 0 {
            println!("Invalid Option!\n");
            continue;
        }

        question = question.answer[choice_i - 1].clone();
    }
}
