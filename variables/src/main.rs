fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("The const is {}", MAX_POINTS);
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    // This creates an immutable variable y
    let y = 5;
    // This is shadowing.  It is when you reinitialize the variable to be able
    // to assign a new value to it.  This replaces the old variable with the
    // new, so the next time y is called, this will be used, not the original.
    // The primary benefit of this is that the variable can only be changed
    // when the 'let' keyword is used.  Also, this allows us to change the data
    // type without creating another variable name
    let y = y + 1;
    let y = y + 2;
    println!("The value of y is {}", y);
    // The underscore is used to prevent compiler warnings
    // Addition:
    let _sum = 5 + 10;
    // Subtraction:
    let _diff = 195.5 - 4.3;
    // Multiplication:
    let _product = 4 * 30;
    // Division:
    let _quotient = 56.7 / 32.2;
    // Remainder:
    let _remainder = 43 % 5;
    // Boolean:
    let _t = true;
    let _f: bool = false;

    // There are two primitive compound types: tuples and arrays.  These group
    // multiple values into a single type
    // Tuples:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // We can de-structure tuples like so:
    let (_x, y, _z) = tup;
    println!("The value of y is {}", y);
    // We can also access elements like so:
    println!("The third value of tup is {}", tup.2);
    // Arrays:
    // These have fixed length and each element must be the same type
    // If the length should be variable, use a vector
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // You can also create an array with all the same elements like so
    // ([3, 3, 3, 3, 3]):
    let _b = [3; 5];
    // To access elements of an array:
    let _second = a[1];

    another_function(5, 6);
    state_vs_exp();

    let f = five();
    println!("The function five returns {}", f);

    let s = plus_one(5);
    println!("plus_one turns 5 to {}", s);
}

// It makes no difference if the function is written before or after where it is
// called.  Also, you must include typing
fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is {} and y is {}", x, y);
}

// Entire function definitions are statements.  Statements do not return values
// Functions and macros are expressions though
fn state_vs_exp() {
    // Creating a variable and assigning it a value is a statement
    // Expressions can be part of statements, like 'x = 6' within the 'let'
    let mut x = 6;
    x = x + 1;

    let y = {
        // This variable does not leave the scope of the brackets
        let x = 3;
        // Expressions evaluate to a value and do not include a semicolon
        // It becomes a statement if you include the semicolon
        x + 1
    };
    println!("The value of y is {} and x is {}", y, x);
}

// This is a function that returns a value
fn five() -> i32 {
    5
}

// This returns a modified value.  Semicolons turn expressions into statements,
// and since statements don't return a value, we must have an expression to
// return a value in the function
fn plus_one(x: i32) -> i32 {
    x + 1
}
