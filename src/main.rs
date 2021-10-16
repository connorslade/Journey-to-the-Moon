use std::env;
use std::fs;

#[macro_use]
mod common;
mod local_game;
mod parse;
mod server;

const VERSION: &str = "0.0.0";
const TREE_PATH: &str = "data/tree.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    let web_server = args.contains(&"--web".to_string());

    // TODO: Load from Config
    let ip = "0.0.0.0";
    let port = 8081;

    println!("[*] Starting Journey to the Moon (V{})", VERSION);

    let raw_data = time_print!("[*] Reading Tree", || fs::read_to_string(TREE_PATH)
        .unwrap()
        .replace('\r', ""));
    let data = time_print!("[*] Parseing Tree", || parse::Question::parse_json(
        raw_data
    )
    .unwrap());

    if web_server {
        println!("[*] Starting Server ({}:{})\n", ip, port);
        server::start_server(ip, port);
        return;
    }

    local_game::local_hame(data);
}
