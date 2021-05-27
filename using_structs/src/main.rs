fn main() {
    using_structs();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn using_structs() {
    // We can only make entire instances of structs mutable, not individual
    // values within it
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("some_username"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another");

    let email = String::from("an_email@email.com");
    let username = String::from("some_user");
    let user2 = build_user(email, username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // email: email, // This is shorthand for this
        username, // username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Pick up at: Creating Instances From Other Instances With Struct Update Syntax
