use std::env;
use std::process;

use lidl_grep::Config;

fn main() {
    let config = Config::setup(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse args: {err}");
        process::exit(1);
    });

    if let Err(e) = lidl_grep::run(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
