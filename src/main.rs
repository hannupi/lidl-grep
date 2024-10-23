use std::env;
use std::process;

use lidl_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::setup(&args).unwrap_or_else(|err| {
        println!("Failed to parse args: {err}");
        process::exit(1);
    });

    if let Err(e) = lidl_grep::run(config) {
        println!("Error: {e}");
        process::exit(1);
    }
}
