use std::error::Error;
use std::fs;

pub struct Config {
    // Using primitive values when a complex type is more appropriate is an
    // anti-pattern known as 'primitive obsession'
    pub query: String,
    pub filename: String,
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
        Ok(Config { query, filename })
    }
}

// Learn more about trait objects in Ch 17
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);

    // Putting the '()' within the 'Ok()' is the idomatic way to do it.  It says
    // that we aren't looking for any return values from this method
    Ok(())
}
