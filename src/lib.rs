use std::{fs, error::Error, env}; // if your program needs to accept arguments containing invalid Unicode, use std::env::args_os

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            3 => {
                let file_path: String = args[1].clone();
                let query: String = args[2].clone();
                let ignore_case = env::var("IGNORE_CASE").is_ok();
                Ok(Config { query, file_path, ignore_case })
            },
            4 => {
                let file_path: String = args[1].clone();
                let query: String = args[2].clone();
                let ignore_case_env = env::var("IGNORE_CASE").is_ok();
                let ignore_case_arg = args[3].clone();
                let mut ignore_case = false;
                if (ignore_case_env || ignore_case_arg == "y") == true {
                    ignore_case = true;
                }
                Ok(Config { query, file_path, ignore_case })
            },
            _ => Err("not enough arguments"),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    println!("{}", content);
    let res;
    if config.ignore_case {
        res = search_case_insensitive(&config.query, &content);
    }
    else {
        res = search(&config.query, &content);
    }
    println!("{:?}", res);
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
