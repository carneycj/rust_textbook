// This project will have the basic functionality of 'grep', or 'Globally search
// a Regular Expression and Print'

use std::env;
use std::process;

use io_project::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // This makes the code exit with code 1.  Typically, exiting with a
        // non-zero code indicates an error
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // We use if let rather than unwrap_or_else because we don't care about the
    // success, only the error case.
    if let Err(e) = io_project::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
