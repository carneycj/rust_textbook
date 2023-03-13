fn main() {
    let four = IpAddrKindSimple::V4;
    let six = IpAddrKindSimple::V6;

    route(four);
    route(six);

    struct_and_enum();
    discussing_nulls();
    using_match();
    using_if_let()
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

fn using_match() {
    // match is great but if checking only one condition, if let may be better
    let value = value_in_cents(Coin::Penny);
    println!("{}", value);
    let value2 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", value2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    use_placeholder();
}

fn use_placeholder() {
    let some_u8_value = 0u8;
    // match is exhaustive.  This means you need to cover all possible cases
    // One way to do blanket covering is with '_'.  It needs to be the last case
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // Some(5) matches Some(i), so i grabs the value within Some() (so it
        // grabs '5'), and adds 1 to it, leaving it within the wrapper
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    // This is practically the same as 'if', but if needs the expression to
    // evaluate to a boolean, but match allows for any type.  Here we use the
    // type Coin that we just created
    match coin {
        // These are the arms of the match case.  Each arm has two parts: a
        // pattern and some code.  The two parts are split by '=>' and each arm
        // is separated with a comma
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // This is why we needed the debug mode (to print the specific state)
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // to inspect the state
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

fn using_if_let() {
    example_match();
    example_if_let();

    quarter_example(Coin::Nickel);
    quarter_example(Coin::Quarter(UsState::Pennsylvania));
}

fn example_match() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

fn example_if_let() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn quarter_example(coin: Coin) {
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // This is a cleaner way to write the short match case
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
