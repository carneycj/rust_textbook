// RESUME: 13.1, Moving Captured Values Out of Closures and the Fn Traits
use std::thread;
use std::time::Duration;

fn main() {
    shirt_example();

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(5);

    fn_closure_similarities();

    ownership_in_closures();
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn shirt_example() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn fn_closure_similarities() {
    // We can see that functions and closures hve very similar structures
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // Closures don't require type annotations, but they can be included to
    // increase explicitness.  However, if no type snnotation is used, rust
    // still needs to infer the type, so the closure needs to be called.
    // This is why v3 works, but v4 doesn't
    let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;

    // Since v3 uses an inferred type, the first time the closure is called, any
    // type can be used, but once the first one is called, all subsequent calls
    // need to match the type, or else the code won't compile, throwing a type
    // error.
    let ex3_1 = add_one_v3(4);
    // let ex3_2 = add_one_v3("test");
}

fn ownership_in_closures() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Closures can capture values in three ways, which map directly to the
    // three ways a function can take a parameter: borrowing immutably,
    // borrowing mutably, and taking ownership.  The closure decides depending
    // on the body of the function does with the captured values.
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling the closure: {:?}", list);
    only_borrows();
    println!("After calling the only_borrows closure: {:?}", list);

    // We need to print before defining the closure because we can't do a
    // mutable and immutable reference at the same time.  The defining of the
    // closure creates the mutable reference, and then after the final time
    // the closure is called, the mutable borrow ends.  This is what allows us
    // to print list2 again the line after the last call of the closure.
    let mut list2 = vec![1, 2, 3];
    println!("Before defining the closure: {:?}", list2);
    let mut borrows_mutably = || list2.push(list2.last().unwrap() + 1);
    borrows_mutably();
    println!("After calling the borrows_mutably closure: {:?}", list2);

    // If you want to force the closure to take ownership of the value, even if
    // the body of the closure doesn't strictly need ownership, you can use the
    // 'move' keyword before the parameter list
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
