fn main() {
    // We can use generics to allow us to reduce code duplication by allowing
    // different types to use the same code
    // Traits can be used to constrain the types used in generics

    exploring_generics();
}

fn exploring_generics() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 8954, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 's', 'q', 't', 'b', 'n'];
    let result = largest_char(&char_list);
    println!("The largest number is {}", result);

    // let number_list = vec![102, 34, 6000, 8954, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    // let char_list = vec!['a', 's', 'q', 't', 'b', 'n'];
    let result = largest_char(&char_list);
    println!("The largest number is {}", result);

    // This struct design only supports having both the same type
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let diff = Point {x: 5, y: 4.0};

    // This struct design supports different types
    let integer2 = Point2 { x: 5, y: 10 };
    let float2 = Point2 { x: 1.0, y: 4.0 };
    let diff2 = Point2 { x: 5, y: 4.0 };

    // Generics are helpful, but if you use too many in one function, you may
    // want to find a way to break it into smaller bits for readability's sake

    use_point();
    use_point2();

    are_gens_slow();
}

// Turning this into a function reduces repetition in the code.
// We have it set up so that any slice of i32 values can be used.
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    // &item isn't a reference to a reference.  It is pattern matching and
    // destructuring each &i32 value, so that instead item is an i32 inside the
    // for loop.  Pattern matching is in Ch 18
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// We are doing the same thing but for characters.  Seems to be a waste, writing
// the same thing twice.  This is why we use generics
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// We read this definition as: the function 'largest' is generic over some type
// 'T'.  This function has one parameter named 'list', which is a slice of
// values of type 'T'.  The 'largest' function will return a value of the same
// type 'T'
// fn largest<T>(list: &[T]) -> T {
// let mut largest = list[0];

// for &item in list {
//     if item > largest {
//         largest = item;
//     }
// }
// largest
// }

// We can also apply generics to structs:
// In this case, x and y must be the same type
struct Point<T> {
    x: T,
    y: T,
}

// This supports different types between x and y
struct Point2<T, U> {
    x: T,
    y: U,
}

// We have to use the <T> right after impl to specify that we're implementing
// methods on the type Point<T>.  Doing this is what tells the compiler that T
// is a generic and not a concrete type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This one uses a concrete type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn use_point() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

impl<T, U> Point2<T, U> {
    // Some generics are defined at the impl level, others at the fn level
    // We use V and W at this level because it only applies to this function
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn use_point2() {
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// We can also do this with enums:
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn are_gens_slow() {
    // Generics do not slow down rust
    let integer = Some(5);
    let float = Some(5.0);

    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
