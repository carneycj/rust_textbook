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
    // Addition:
    let sum = 5 + 10;
    // Subtraction:
    let diff = 195.5 - 4.3;
    // Multiplication:
    let product = 4 * 30;
    // Division:
    let quotient = 56.7 / 32.2;
    // Remainder:
    let remainder = 43 % 5;
    // Boolean:
    let t = true;
    let f: bool = false;

    // There are two primitive compound types: tuples and arrays.  These group
    // multiple values into a single type
    // Tuples:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // We can de-structure tuples like so:
    let (x, y, z) = tup;
    println!("The value of y is {}", y);
    // We can also access elements like so:
    println!("The third value of tup is {}", tup.2);
    // Arrays:
    // These have fixed length and each element must be the same type
    // If the length should be variable, use a vector
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // You can also create an array with all the same elements like so
    // ([3, 3, 3, 3, 3]):
    let b = [3; 5];
    // To access elements of an array:
    let second = a[1];

    another_function();
}

// It makes no difference if the function is written before or after where it is
// called
fn another_function() {
    println!("Another function.");
}

// Pick up at Function Parameters
