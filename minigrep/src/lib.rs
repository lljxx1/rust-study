use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[2].clone();
        let filename = args[1].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        println!("{}", case_sensitive);

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config.filename);
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}


pub fn search<'a>(query: &str, conent: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in conent.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}


pub fn search_case_insensitive<'a>(query: &str, conent: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in conent.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let conent = "\
Rust;
safe, fast, productive.
Pick There
";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, conent)
        );

    }


    #[test]
    fn test_search_case_insensitive() {
        let query = "duct";
        let conent = "\
Rust;
safe, fast, productive.
Pick There
";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, conent)
        );
    }
}