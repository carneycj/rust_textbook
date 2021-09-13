// This is a library crate.  It has the same name as the package
// We can create it using "cargo new --lib <project name>"

// Modules:
// Modules can hold definitions for items, such as structs, enums, constants,
// and traits, in addition to functions and other modules.
// Modules group related definitions together and reveal how they're related.

// The semicolon indicates that the module must be loaded in from a separate
// file.  The separate file must be named the same as the module being loaded.
mod front_of_house;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // The "super" keyword takes you back one level, allowing relative
        // access to parent modules
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // We must create a public associated function summer for the struct
        // Breakfast here, since seasonal_fruit is private and would prevent us
        // from creating instances of Breakfast otherwise
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // If an enum is public, all variants are public by default.  Therefore, we
    // can create instances outside this module without initializing it here
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// eat_at_restaurant and front_of_house are 'siblings' in terms of the module
// tree.  This means that this function can access the module even though it
// isn't public
pub fn eat_at_restaurant() {
    // To show paths to modules, we can use either absolute or relative paths.
    // The general rule of thumb is to consider if the two are more likely to be
    // moved individually or together.  Absolute is better for individually,
    // relative is better for together.

    // Absolute: The path starting from the crate root by using the crate name or
    // the literal "crate"
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative: The path starting from the current module and uses "self",
    // "super", or some other identifier of the current module.  Here we start
    // with the name of the module level at the same level as eat_at_restaurant
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with rye toast:
    let mut meal = back_of_house::Breakfast::summer("rye");
    // Change our mind about what toast we want to eat:
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't work since we can't see or modify this attribute:
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    path_scope_ex();
}

// This is called "Re-exporting".  Using the 'pub' keyword allows external code
// to call "hosting::add_to_waitlist".
pub use crate::front_of_house::hosting;

pub fn path_scope_ex() {
    // This makes hosting a valid name in this scope, allowing us to not need to
    // write out the full path every time we call it.
    // Paths brought into scope with "use" also check privacy, like any other
    // path.
    use crate::front_of_house::hosting;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Relative paths can also be used
    // While going all the way to the function is possible, it's only idiomatic
    // for structs and enums.  Functions should designed show the parent module
    // use self::front_of_house::serving;
    // serving::serve_order();
    // This shows that the function isn't locally defined while also minimizing
    // verboseness.
    use self::back_of_house::Appetizer;
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    // The 'as' keyword allows you to create an alias for something that already
    // exists.  This allows you to work around naming conflicts like below:
    // use std::fmt::Result;
    // use std::io::Result as IoResult;
    // fn function1() -> Result {}
    // fn function2() -> IoResult<()> {}

    // An alternative is to just use the parent modules to provide clarity:
    // use std::fmt;
    // use std::io;
    // fn function1() -> fmt::Result {}
    // fn function2() -> io::Result<()> {}

    // Both options are idiomatic

    // You can do nested paths.  This cleans up long lists of import statements:
    // use std::cmp::Ordering;
    // use std::io;
    // use std::io::Write;
    // becomes:
    use std::{
        cmp::Ordering,
        io::{self, Write},
    };

    // There is also a glob operator (*).  It brings all public items defined
    // within the path into scope.  This really isn't a great practice though
    // use std::collections::*;
}
