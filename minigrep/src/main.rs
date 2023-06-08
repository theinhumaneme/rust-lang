use std::env;
use std::error::Error;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        panic!("Not enough arguments");
    }
    let config = parse_args(&args);
    run(config);
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_args(args: &[String]) -> Config {
    let query = args[0].clone();
    let file_path = args[1].clone();
    Config { query, file_path }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_data = fs::read_to_string(config.file_path)?;
    Ok(())
}
