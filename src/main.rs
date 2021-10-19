#![allow(clippy::redundant_closure_call)]

use std::env;
use std::fs;

use simple_config_parser::config::Config;

#[macro_use]
mod common;
mod local_game;
mod parse;
mod server;
use parse::Question;

const VERSION: &str = "0.0.0";
const TREE_PATH: &str = "data/tree.json";

// ... much bodge
pub static mut DATA: Option<Question> = None;

fn main() {
    let web_server = env::args().any(|x| x == *"--web");

    let mut cfg = Config::new(Some("data/config.cfg"));
    cfg.read().ok().expect("Error reading the config file");

    let ip = cfg.get("ip").unwrap();
    let port = cfg.get("port").unwrap().parse::<u16>().unwrap();

    println!("[*] Starting Journey to the Moon (V{})", VERSION);

    let raw_data = time_print!("[*] Reading Tree", || fs::read_to_string(TREE_PATH)
        .unwrap()
        .replace('\r', ""));
        let data = time_print!("[*] Parseing Tree", || Question::parse_json(raw_data)
        .unwrap());

    unsafe { DATA = Some(data.clone()) }

    if web_server {
        println!("[*] Starting Server ({}:{})\n", ip, port);
        server::start_server(&ip, port);
        return;
    }

    local_game::local_hame(data);
}
