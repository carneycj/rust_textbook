fn main() {
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
}
