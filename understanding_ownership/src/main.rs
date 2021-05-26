fn main() {
    // _s is not valid yet, since it has not been declared
    // This is a string literal
    let _s = "hello"; // Now it's declared, so it is valid from now on
    string_type();
    data_interactions();
    return_vals_and_scope();
    using_references();
} // _s is not valid, because it is out of scope now

// Here we show the String type and why it's useful
fn string_type() {
    // This is a String type.  It is mutable
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    // String literals are nice because they use their forced immutability to
    // greatly improve performance.  The immutability is required in order to
    // use stack storage rather than heap storage.  The String type is nice
    // because it allows you to create mutable strings, at the cost of some
    // performance.  This is because it uses heap storage rather than stack.
}

fn data_interactions() {
    let x = 5;
    // This creates a copy of x (even if we didn't use mut in the let statement)
    let mut y = x;
    y += 1;
    println!("x: {}, y: {}", x, y);

    // The String type has three components-
    // 1: Pointer to the actual value of the string (the actual value of the
    //      string is in the stack)
    // 2: The length of the string
    // 3: The capacity of the string
    let s1 = String::from("hello");
    // With the declaration of s2, s1 is now invalid.  This is because of the
    // pointer component in the String type.  This is similar to a shallow copy,
    // but is better described as a move, since the original variable is
    // destroyed.  This is a fast operation
    let s2 = s1;
    // This is a deep copy.  It creates a new pointer to a different location
    // with a copy of the actual string data.  This can be an expensive
    // operation
    let s3 = s2.clone();
    println!("{}, {}", s2, s3);

    takes_ownership(s3); // s3's value moves into the function
                         // ... and is now no longer valid
    makes_copy(x); // x would move into the function, but can be copied
                   // ... so x is still valid here
}

fn takes_ownership(var: String) {
    // Some string (var) comes into scope
    println!("{}", var);
} // Now var (the string) is out of scope, so the backing memory is dropped

fn makes_copy(var: i32) {
    // Some integer (var) comes into scope
    println!("{}", var);
} // Now var (the int) is out of scope.  Because it is an int, nothing happens

fn return_vals_and_scope() {
    let s1 = gives_ownership(); // The function's return is moved to s1
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved to the function, and the
                                       // function's return is moved to s3
                                       // After this function (data_interactions), s3 goes out of scope and is
                                       // dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes
                                       // out of scope and is dropped
    println!("{}, {}", s1, s3);
    let (s4, length) = returns_tuples(s3);
    println!("{}, {}", s4, length);
}
fn gives_ownership() -> String {
    // This takes in no arguments and returns a string that will be moved into
    // the function that called this one
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // This takes in a string and returns that same string, which will be moved
    // into the function that called this one
    a_string
}

fn returns_tuples(s: String) -> (String, usize) {
    // We can also return tuples when we want to return multiple
    let length = s.len();
    (s, length)
}

fn using_references() {
    let s1 = String::from("hello");
    // Using the & makes this a reference to s1, meaning the function doesn't
    // take ownership of the variable. This is important because it doesn't
    // delete the variable once out of scope. This is called borrowing
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
}

// It's important to add the & to the type when denoting a reference
fn calculate_length(s: &String) -> usize {
    // s is a reference to a string
    s.len()
} // Since s doesn't own the variable, it does not get deleted once out of scope

// Pick up at mutable references
