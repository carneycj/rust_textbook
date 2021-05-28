fn main() {
    let four = IpAddrKindSimple::V4;
    let six = IpAddrKindSimple::V6;

    route(four);
    route(six);

    struct_and_enum();
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

// COntinue at: The Option Enum and Its Advantages Over Null Values
