use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use crate::common;
use crate::parse::Question;

/// Char Delay (ms)
const DELAY: u64 = 25;

#[derive(Copy, Clone, Debug)]
struct CharDelay {
    pub char: char,
    pub delay: u64,
}

const CHAR_DELAYS: &[CharDelay] = &[
    CharDelay {
        char: '.',
        delay: 150,
    },
    CharDelay {
        char: ':',
        delay: 250,
    },
    CharDelay {
        char: '\n',
        delay: 500,
    },
];

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

        // *THE* nested condtional
        // 🏆️
        if question.end.is_some() {
            if question.end.unwrap() {
                delay_print(&format!("{}You Win!", to_print));
                return;
            }
            delay_print(&format!("{}You Lose...", to_print));
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

        to_print.push_str("\n\n");

        delay_print(&to_print);
        let choice = input!("> ");
        if common::contains_any(&choice.to_lowercase(), &["exit", "quit", "go away"]) {
            return;
        }
        println!();

        let choice_i = match choice.parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                delay_print("Invalid Option!\n\n");
                continue;
            }
        };

        if choice_i > question.answer.len() || choice_i == 0 {
            delay_print("Invalid Option!\n\n");
            continue;
        }

        question = question.answer[choice_i - 1].clone();
    }
}

fn delay_print(text: &str) {
    for i in text.chars() {
        print!("{}", i);
        stdout().flush().expect("Err Flushing STD Out");
        sleep(Duration::from_millis(get_delay(i)));
    }
}

fn get_delay(char: char) -> u64 {
    for i in CHAR_DELAYS {
        if i.char == char {
            return i.delay;
        }
    }
    DELAY
}
