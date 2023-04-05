use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use itertools::Itertools;
use std::{
    env::{self, Args},
    fs,
};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(query: String, file_path: String, ignore_case: bool) -> Self {
        Self {
            query,
            file_path,
            ignore_case,
        }
    }

    pub fn build(args: Args) -> Result<Self> {
        let (query, file_path) = args
            .skip(1)
            .next_tuple()
            .ok_or_else(|| eyre!("Expected two arguments: query, file_path"))?;

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self::new(query, file_path, ignore_case))
    }
}

pub fn run(config: Config) -> Result<()> {
    let contents =
        fs::read_to_string(&config.file_path).wrap_err(format!("Reading: {}", config.file_path))?;

    search(&config.query, &contents, config.ignore_case)
        .iter()
        .for_each(|line| println!("{line}"));

    Ok(())
}

fn search<'a>(query: &str, content: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let format_str = |str: &str| -> String {
        if ignore_case {
            str.to_lowercase()
        } else {
            str.to_string()
        }
    };

    let query = format_str(query);

    content
        .lines()
        .filter(|line| format_str(line).contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
