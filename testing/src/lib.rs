// To run tests, use 'cargo test'

fn info() {
    // A test is a function that's annotated with the 'test' attribute
    // Attributes are metadata about pieces of rust code
    // You create test function by adding #[test] the line above the function
    // definition
    println!("Hello, world!");

    // cargo test has command line options that can go into 'cargo test' or the
    // resulting test binary.
    // 'cargo test': cargo test --help
    // Resulting Binary: cargo test -- --help

    // Since the tests are run in parallel on separate threads, tests cannot
    // depend on one another or on any shared state, including a shared
    // environment, such as the current working directory or environment
    // variables.  Ways to work around this are to create unique files for each
    // test, or to make only one thread do all of the tests using the command
    // cargo test -- --test-threads=1

    // Rust's testing mentality is to use unit tests and integration tests.
    // Unit tests are small and focused, testing individual functions and can
    // test private interfaces.  Integration tests are entirely external to your
    // library and use your code in the same way any other external code would,
    // using only public interfaces and potentially using multiple modules per
    // test.  It's important to use both, making sure that your functions and
    // modules work both individually and together

    // You can run tests by the module, which would be done with the command
    // 'cargo test <module name>::'

    // Unit Tests go in the src directory in the same file as the code that it
    // is testing.  The convention is to create a module named tests in each
    // file to contain the test functions and to annotate the module with
    // #[cfg(test)].  That annotation on the tests module tells Rust to only
    // compile and run the code when testing, not running.

    // Integration Tests are entirely external to the library.  They use the
    // library the same way any external code would use it.  This means that the
    // tests can only access public methods.  Individual methods can work fine,
    // but create issues once working together.  All integration tests need to
    // be in the tests directory.  This is at the same level as src.  In it, as
    // many test files can be created

    // Integration tests can only be run on library functions/ modules.  Binaries
    // don't have this functionality because they don't expose their functionality
    // externally.  This is why binaries typically will have a simple 'main.rs' and
    // then pull in functionality from the libraries.  This lets you do integration
    // testing while still creating a proper application.  The small amount of code
    // in main won't need full testing.
}

#[cfg(test)]
mod tests {
    // We need this line to be able to access the other items.  The tests module
    // is a normal module that follows all the normal visibility rules of module
    // trees.
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn fail_test() {
        // Each test is run on a new thread, and if a thread dies, the test is
        // marked as failed
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // This lets us make sure that this won't work
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // The order doesn't matter
        assert_eq!(4, add_two(2));
        assert_eq!(add_two(2), 4);
        // You can also use != for operations where you don't know what it would
        // be, but do know what it won't be
        assert_ne!(add_two(2), 6);
        assert_ne!(2, add_two(2));
        // The values compared with these two macros must implement the Debug
        // and PartialEq traits.  Usually all this means is that you need to add
        // #[derive(Debug, PartialEq)]
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // This is useful when we only want to check certain pieces, because we
        // don't want to have to remake this every time something small changes
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        // Here, using the #[should_panic] tells the test that it should fail.
        // This is not very precise, since it doesn't show why the test failed
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_clearer() {
        // This is more precise, since we tell it what to expect.  Here, we used
        // a substring of the output we expect.  We could have also used the
        // entire panic message, but this is more flexible
        Guess::new(200);
    }

    #[test]
    fn it_works_too() -> Result<(), String> {
        // Using Result in our tests allows us to use ? in the test body (making
        // things more convenient).  In this case, you shouldn't use the
        // #[should_panic] annotation, but rather return the Err value directly
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        // Passed tests won't return any print lines, just an 'ok'
        // If you want rust to not capture the output, use '
        // cargo test -- --show-output'
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        // Failed tests will return strings to help troubleshoot
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }

    #[test]
    fn two_add_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn five_add_two() {
        // We can run tests on all tests with a subset of a string in the name
        // too with 'cargo test add'
        assert_eq!(add_two(5), 7);
    }

    #[test]
    fn one_hundred() {
        // We can run just one test with 'cargo test one_hundred'
        assert_eq!(add_two(100), 102);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // We can also make it so that a test only runs if we specifically ask
        // for it using #[ignore].  This is useful for tests that are really
        // slow and so only want to run when making changes that could affect it
        // If we want to run all ignored tests, use 'cargo test -- -- ignored'
        assert_eq!(add_two(5), 7);
    }

    #[test]
    fn test_private() {
        // You can test private functions too.  Test modules are normal modules
        // just with a test attribute, so it can access other methods in the
        // library like normal
        assert_eq!(internal_adder(2, 2), 4);
    }

    #[test]
    fn internal() {
        assert_eq!(internal_adder(2, 2), 4);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // If we use < instead of > for one of them, the test would catch it
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}

/// This is a Doc string.  You use three /s.  Adding 3 "`" right after denotes
/// a doc test:
///```
/// let result = testing::prints_and_returns_10(3);
/// assert_eq!(result, 10);
/// ```
pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

/// Another doc string (you can pass arguments in if you lead it with 'rust,'
/// ```rust,ignore
/// let result = testing::prints_and_returns_10_too(5);
/// assert_eq!(result, 5);
/// ```
pub fn prints_and_returns_10_too(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
