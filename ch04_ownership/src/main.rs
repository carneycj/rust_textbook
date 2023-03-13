fn main() {
    // _s is not valid yet, since it has not been declared
    // This is a string literal
    let _s = "hello"; // Now it's declared, so it is valid from now on
    string_type();
    data_interactions();
    return_vals_and_scope();
    using_references();
    understanding_slicing();
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
    println!("\nx: {}, y: {}", x, y);

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
    println!("\n{}, {}", s1, s3);
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
    println!("\nThe length of {} is {}", s1, len);

    let mut s = String::from("hello");
    // This is how we allow a reference to make changes to the variable that
    // it's borrowing.
    change(&mut s);

    // We also can't have a mutable reference used at the same time as immutable
    // references.  The immutable references don't expect the value to change
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // This is fine because we don't use the immutable references any more
    let r3 = &mut s;
    println!("{}", r3);
}

// It's important to add the & to the type when denoting a reference
fn calculate_length(s: &String) -> usize {
    // s is a reference to a string
    s.len()
} // Since s doesn't own the variable, it does not get deleted once out of scope

// We need to include the &mut, since we're changing the value
fn change(some_string: &mut String) {
    // We can't have mutable references to a single variable at the same time
    // This prevents data racing, which is when:
    // - Two or more pointers access the same data at the same time
    // - At least one of the pointers is being used to write the data
    // - There's no mechanism being used to synchronize access to the data
    some_string.push_str(", world");
}

fn understanding_slicing() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    // my_string.clear(); // This needs mut
    // clear makes my_string = "", so the length of the first is no longer 5,
    // but without string slicing, word doesn't change, and there is no compile
    // error
    println!("\n{}", word);
    // A better solution is string slicing:
    let s: String = String::from("hello world");
    // This is a reference to a portion of a string. It is [start: end + 1]. If
    // the start = 0 or if the end = length of string, that value can be dropped
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let _word = first_word(&my_string[..]);

    let my_string_literal = "hello world";
    let _word = first_word(&my_string_literal[..]);
    let _word = first_word(my_string_literal);

    // We can also slice arrays:
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
}

fn first_word(s: &str) -> &str {
    // Use type usize for not slicing
    // This function finds the first word in a string
    // In order to search the string for a character, we convert to an array of
    // bytes and iterate through it
    let bytes = s.as_bytes();

    // The first item in the tuple of enumerate is the index, and the second is
    // a reference to the item that the iter method returns (so we need the &)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i; // This is how it would be done without slicing
            return &s[0..i];
        }
    }

    // If there is no space, the length of the entire string is returned
    // s.len() // If slicing wasn't used
    &s[..]
    // The return value of this function is literally just the index of the end of
    // the first word.  If the string changes, this function doesn't change, so
    // everything goes wrong, even though this doesn't create a compile error
}
