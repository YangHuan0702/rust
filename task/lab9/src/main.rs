use std::env;
use std::fs;

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(query: String, file_name: String) -> Config {
        Config { query, file_name }
    }
}

fn main() {
    let params: Vec<String> = env::args().collect();
    let config : Config = set_config(&params);
    let context = fs::read_to_string(config.file_name).expect("read file error");

    for x in context.lines() {
        if x.contains(&config.query) {
            println!("{}",x);
        }
    }
}


fn set_config(params: &Vec<String>)-> Config {
    let a = params[1].clone();
    Config::new(params[1].clone(),params[2].clone())
}