fn main() {
    using_loop();
    using_while();
    using_for();
}

fn using_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn using_while() {
    println!("while loops:");
    // This can also be accomplished using loop, if, else, and break, but this
    // is a cleaner implementation
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // This isn't really a good way to do this though because we could
    // accidentally go for an index that doesn't exist, and it's a slow way to
    // do it (for is faster)
    while index < 5 {
        println!("The value is {}", a[index]);
        index += 1;
    }
}

fn using_for() {
    println!("for loops:");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    // This is more robust than a while loop (faster and safer)
    for element in a.iter() {
        println!("the value is {}", element);
    }
}
