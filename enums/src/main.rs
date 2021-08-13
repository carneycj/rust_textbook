fn main() {
    let four = IpAddrKindSimple::V4;
    let six = IpAddrKindSimple::V6;

    route(four);
    route(six);

    struct_and_enum();
    discussing_nulls();
}

enum IpAddrKindSimple {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKindSimple,
    address: String,
}

enum IpAddr {
    // You can even use structs or enums as the types for each component.  Even
    // no data type can be included
    V4(u8, u8, u8, u8),
    V6(String),
}

// This is the key advantage of the enum: it allows us to say any type of that
// group can be used
fn route(ip_kind: IpAddrKindSimple) {}

fn struct_and_enum() {
    // Here we can see that we can use structs and enums together.  However,
    // this is a bit verbose
    let home = IpAddrStruct {
        kind: IpAddrKindSimple::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKindSimple::V6,
        address: String::from("::1"),
    };

    // The enum can do this without a struct and makes things more concise:
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// These all could hold the same data as the enum listed above (Message) but
// using enum here is better, since calling functions with all of these would be
// more difficult than if using enum
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body goes here
    }
}

// Null isn't implemented the way most other languages do.  It isn't a type, but
// instead is an enum that allows for the concept of a value being present or
// absent.  This is in the standard library:
enum ExampleOption<T> {
    // This enum and its variants (Some(T) and None) are all included in the
    // prelude.  This means that you don't even need to use Option:: to call
    // it.  <T> is similar to Some(T).  The variable T can be one value of
    // one type
    Some(T),
    None,
}

fn discussing_nulls() {
    // The type of a variable can be inferred when there is actually a value
    let some_number = Some(5);
    let some_string = Some("a string");
    // Option can't infer the type of a blank value, so we have to explicitly
    // include it
    let absent_number: Option<i32> = None;

    // You can't combine a type with an Option<type>.  You would need to use
    // different built-in methods for Option, or you can use a 'match' expression
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
}

// Continue at: The match Control Flow Operator
