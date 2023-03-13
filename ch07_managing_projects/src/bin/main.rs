fn main() {
    // A package must contain at least one crate.
    // A crate is defined as either a binary (bin) or library (lib).  A package
    // may have any number of bins, but may not have more than one lib.

    // "cargo new managing_projects" created the package managing_projects

    // src/main.rs is the crate root.  It has the same name as the package
    println!("Hello, world!");
}
