// We have to add this dependency in the Cargo.toml file
use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // rand::thread_rng() creates a random number generator.  We use .gen_range
    // to actually create a specific random number.  We can do (1..101) or
    // (1..=100), both give a range [1, 100].  The type is inferred from the
    // compare later in this method
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");
    loop {
        println!("Please input your guess.");

        // Creates a new, mutable empty string called guess
        let mut guess = String::new();

        io::stdin()
            // Reads the user input and appends it to guess.  The & means that
            // this is a reference to the original variable location.
            // References are also default immutable, so in order to append the
            // data, we use mut
            .read_line(&mut guess)
            // This provides info to help understand errors.  This is a good
            // step but actual error handling is better
            .expect("Failed to read line");

        // This shadows the previous value of guess with a new one.  We also are
        // changing the type from a string into an int.  If the user input is
        // invalid, then we continue (skip this loop iteration and go again).
        // The _ means any value associated with the tag Err is accepted
        // You can also to trim().parse().expect("Please type a number") instead
        // of doing a match, but parse will panic and end the script with the
        // message presented in expect
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // .cmp is a compare method (greater than, less than, or equal to)
        // We can also add color to our command line with colorize
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
