use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Rust differentiates between recoverable and unrecoverable errors.
    // Recoverable errors are ones that typically just need the user to try
    // again, like 'file not found'.  The way rust handles this is with the type
    // Result<T, E>.  Unrecoverable errors are ones that cause the program to
    // need to shut down because it can't be fixed on the fly.  For this Rust
    // uses the panic! macro.

    // panic! 'unwinds' by default.  This means that it walks back up the stack
    // and cleans up the data from each function it encounters.  This does take
    // work though.  If you want the program to just abort, you can add this to
    // the .toml file:
    // [profile.release]
    // panic = 'abort'

    // ex_simple_panic();
    // ex_panic_backtrace();
    // basic_result();
    // advanced_result();
    // panic_shortcuts();
    // propagating_errors();
    when_to_panic();
    protecting_your_code();

    // The main() function has restrictions on what it can return.
    // Box is something covered in ch 17
    let f = File::open("hello.txt")?;
    Ok(())
}

fn ex_simple_panic() {
    panic!("Crash and burn");
}

fn ex_panic_backtrace() {
    let v = vec![1, 2, 3];
    // Since this is out-of-bounds of v, this will cause the program to panic
    v[99];
    // Rust says that you can add a backtrace by running the script again like
    // this in the terminal: <RUST_BACKTRACE=1 cargo run>
    // This only works if you don't have the --release flag active
}

fn basic_result() {
    // The Result type is: Result<T, E>.  This enum looks like:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    // }
    // use std::fs::File;
    use std::io::ErrorKind;
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        },
    };
}

fn advanced_result() {
    // This way of doing it is much more concise, but requires closures (ch 13)
    // use std::fs::File;
    use std::io::ErrorKind;
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn panic_shortcuts() {
    // use std::fs::File;
    // Unwrap has panic built into it
    // Since the file doesn't exist, unwrap runs the panic macro
    // let f = File::open("hello.txt").unwrap();

    // Expect also has panic built into it
    // Since the file doesn't exist, it runs the panic macro.  One difference
    // though, is that expect() also allows us to include a custom error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

use std::fs::{self, File};
use std::io::{self, Read};
fn propagating_errors() {
    // Instead of immediately handling the error, you can return the error to
    // the calling code so that it can decide what to do.  This is called
    // propagating error

    let res = read_username_from_file();
    let res = err_prop_shortcut();
    let res = even_shorter_read();
    let res = shortest_read();
    let res = using_q_oper();
}

fn read_username_from_file() -> Result<String, io::Error> {
    // This function is designed to return the error information if it goes
    // wrong.  If everything works fine, it gives the original data and an "Ok".
    // If anything goes wrong, it will provide "Err" and more information on
    // what went wrong.  You may need to think about what 'error' you want to
    // use as your return type (eg io::Error)
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn err_prop_shortcut() -> Result<String, io::Error> {
    // This method does the same thing as read_username_from_file() but uses the
    // ? operator to make things easier to read
    // The ? operator works pretty much the same way as the match that returns
    // either the Ok status and the information, or the Error status and error
    // information.  The primary difference is that errors go through the 'from'
    // function (part of From crate).  This converts the error type for you
    // The biggest benefit of the ? operator is the removal of a lot of
    // boilerplate code
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn even_shorter_read() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn shortest_read() -> Result<String, io::Error> {
    // This opens the file, creates a new string, reads the file, puts the
    // contents into that string, and returns it
    fs::read_to_string("hello.txt")
}

fn using_q_oper() {
    // This won't compile because the ? operator can only be used in a function
    // that returns to return either Result, Option, any other type that
    // implements std::ops::try.  To get around this, you can either make the
    // function return one of those two types, or use match or one of the
    // Result<T, E> methods to handle it properly
    // The main() function is special.  It has restrictions on what its return
    // types can be.  Two acceptable types are () and Result<T, E>.  This
    // program's main() has the Return type implemented
    // let f = File::open("hello.txt")?;
}

fn when_to_panic() {
    // Deciding between panic! and Result: Result gives the program options, and
    // panic! just shuts it down.  If you think there could be a way to recover,
    // allowing the program to try and fix it, and then if need be it can shut
    // on it's own as well.  Result is a good default choice

    // unwrap() and expect() are good placeholders for handling errors until you
    // go back in later and make the code more robust

    // unwrap() can also be used properly when something will definitely work
    // but will still return the value within an Ok() that the compiler doesn't
    // know how to use.  For instance, home is a valid IP Address, so it's ok to
    // use unwrap, since it can't fail.
    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

fn protecting_your_code() {
    // Creating your own type also helps to protect your code from errors by
    // ensuring that the input matches what you expect
    // For instance, if you want the user to guess a value [1, 100]:
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess a number in range [1, 100].  You entered {}", value);
            }
            Guess { value }
        }

        // This is called a "getter".  We use this because the struct's value is
        // private.  We want the actual struct field to be private because we
        // want the user to have to use the new() function that we included in
        // this impl to be able to set values directly.  This ensures that all
        // values are within the range that we set
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
