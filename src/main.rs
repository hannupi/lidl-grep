use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filepath = &args[2];
    println!("query: {} filepath: {}", query, filepath);

    let content = fs::read_to_string(filepath).expect("Unable to read filepath");

    println!("{}", content);
}
