use std::error::Error;
use std::{env, fs};
#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let query = args[0].clone();
        let file_path = args[1].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_data = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &file_data, config.ignore_case) {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
safe, fast, productive.
Pick Three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }
}
pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut lines = Vec::new();
    for line in contents.lines() {
        if case_sensitive == true {
            if line.contains(query) {
                lines.push(line);
            }
        } else {
            if line.to_ascii_lowercase().contains(query) {
                lines.push(line);
            }
        }
    }
    lines
}
