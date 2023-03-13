fn main() {
    basic_rectangle();
    tuple_refactor();
    struct_refactor();
    methods_in_structs();
}

fn basic_rectangle() {
    let width1 = 30;
    let length1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, length1)
    );
}
fn tuple_refactor() {
    // A tuple helps group the related data more clearly
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );
}

// We need to add this line in order to print instances of the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

// This is an implementation block to create methods and functions for structs
// You can have multiple impl blocks if you want
impl Rectangle {
    // This is a method because it takes in self as a parameter
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    // This is an associated function because it doesn't take in self as a
    // parameter.  These are commonly used for instance constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn struct_refactor() {
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect1)
    );
    // The :? indicates debugging mode for that printed value
    // :#? prints the value over multiple lines, making structs easier to read
    println!("rect1 is {:?}", rect1);
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}

fn area1(dimensions: (u32, u32)) -> u32 {
    // This is better because the data is grouped, but bad because it's not
    // clear which is length and which is width
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    // This adds the clarity of each variable and groups them
    rectangle.width * rectangle.length
}

fn methods_in_structs() {
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // This is how we call methods.  Rust automatically adds the &, &mut,
        // and * as needed for the instance calling the method to match the
        // signature of the method
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        length: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // We call associated functions with ::
    let sq = Rectangle::square(3);
    println!("The square is: {:#?}", sq);
}
