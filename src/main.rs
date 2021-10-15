use std::fs;

#[macro_use]
mod common;
mod parse;

const VERSION: &str = "0.0.0";
const TREE_PATH: &str = "data/tree.json";

fn main() {
    println!("[*] Starting Journey to the Moon (V{})", VERSION);

    let raw_data = time_print!("[*] Reading Tree", || fs::read_to_string(TREE_PATH)
        .unwrap()
        .replace('\r', ""));
    let data = time_print!("[*] Parseing Tree", || parse::Question::parse_json(
        raw_data
    )
    .unwrap());

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

        let mut index = 0;
        for i in question.clone().answer {
            index += 1;
            to_print.push_str(&format!(
                "{}) {}   ",
                index,
                i.option.unwrap_or("".to_string())
            ))
        }

        println!("{}", to_print);
        let choice = input!("> ");
        println!();

        let choice_i = match choice.parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                println!("Invalid Option!\n");
                continue;
            }
        };

        if choice_i > question.answer.len() {
            println!("Invalid Option!\n");
            continue;
        }

        question = question.answer[choice_i - 1].clone();
    }
}
