fn main() {
    using_structs();
    using_tuple_structs();

    // Unit-like structs don't have fields.  This is useful when implementing
    // traits on some type but don't have any data to store in it
}

struct User {
    // We can use &str, but that requires lifetimes to be able to implement
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
    user1.username = String::from("user5");
    user1.email = String::from("another");
    user1.active = true;
    user1.sign_in_count = 2;

    let email = String::from("an_email@email.com");
    let username = String::from("some_user");
    let _user2 = build_user(email, username);

    let _user2 = User {
        email: String::from("another@email.com"),
        username: String::from("yet_another_username"),
        ..user1 // This is the same as:
                // active: user1.active,
                // sign_in_count: user1.sign_in_count
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // email: email, // This is shorthand for this
        username, // username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn using_tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // black and origin are different types, because they're instances of
    // different tuple structs
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}
