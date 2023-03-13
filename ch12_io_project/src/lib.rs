use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    // Using primitive values when a complex type is more appropriate is an
    // anti-pattern known as 'primitive obsession'
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // You could just panic, but this is more meant for programmers/
            // debugging, rather than the end user
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // This is an environment variable.  We check if it's unset (there's an
        // error).  This would mean a case-sensitive search is desired.  If
        // is_err() returns false, that would mean a case-insensitive search is
        // required.  You can allow arguments and environment variables for the
        // same configuration.  In those cases, the program needs to decide
        // which takes precedence
        // To run: "CASE_INSENSITIVE=1 cargo run to poem.txt"
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Learn more about trait objects in Ch 17
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // Putting the '()' within the 'Ok()' is the idomatic way to do it.  It says
    // that we aren't looking for any return values from this method
    Ok(())
}

// We only put 'a on contents because that's what it should be a reference to
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
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
    fn one_result() {
        let query = "duct";
        // Starting the string with '\' means that there should be no newline
        // character at the beginning of the string
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
