use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments: {err}");
        process::exit(1);
    });
    // dbg!(&config);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

