use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::setup(&args).unwrap_or_else(|err| {
        println!("Failed to parse args: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filepath)?;
    println!("{}", content);

    Ok(())
}

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn setup(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Give at least two args");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { query, filepath })
    }
}
