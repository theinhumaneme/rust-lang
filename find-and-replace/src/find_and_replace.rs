use regex::Regex;
use std::env;
use std::fs;
use std::process::*;
use text_colorizer::*;
#[derive(Debug)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}
fn replace(target: &str, replace: &str, data: &str) -> Result<String, regex::Error> {
    let regex_expression = Regex::new(target)?;
    Ok(regex_expression.replace_all(data, replace).to_string())
}

fn print_help() {
    eprintln!(
        "This is my first program that replaces string with another string :)\nWelcome to {}",
        "Find and Replace".green().bold()
    );
    eprintln!(
        "Usage - {} {} {} {}",
        "TARGET STRING".red(),
        "REPLACEMENT STRING".green(),
        "SOURCE FILE".blue(),
        "OUTPUT FILE".green()
    );
}

fn parse_args() -> Arguments {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.len() != 4 {
        print_help();
        eprintln!(
            "{} Wrong number of arguments given, Expected 4 received {}",
            "Error".red().bold(),
            arguments.len()
        );
        exit(1);
    }
    let parsed_args = Arguments {
        pattern: arguments[0].clone(),
        replace: arguments[1].clone(),
        input_file: arguments[2].clone(),
        output_file: arguments[3].clone(),
    };
    parsed_args
}

fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} Cannot read the file {} :( {:?}",
                "FILE ERROR".red().bold(),
                &args.input_file,
                e
            );
            exit(1);
        }
    };

    let rep = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} Failed to replace text {:?}", "Error".red().bold(), e);
            exit(1);
        }
    };
    match fs::write(&args.output_file, &rep) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} Cannot read the file {} :( {:?}",
                "FILE ERROR".red().bold(),
                &args.input_file,
                e
            );
            exit(1);
        }
    }
}
pub fn run() {
    let args = parse_args();
    read_and_write(&args);
}
