use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    // Rust functional language features

    // Closures:
    // These are anonymous functions that can be saved in a variable or passed
    // as arguments to other functions.  Unlike functions, closures can capture
    // values from the scope in which they're defined
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    println!("generate_workout:");
    generate_workout(simulated_user_specified_value, simulated_random_number);
    println!("generate_workout_refactored_variable:");
    generate_workout_refactored_variable(simulated_user_specified_value, simulated_random_number);
    println!("generate_workout_refactored_closure:");
    generate_workout_refactored_closure(simulated_user_specified_value, simulated_random_number);
    println!("generate_workout_refactored_closure_w_struct:");
    generate_workout_refactored_closure_w_struct(
        simulated_user_specified_value,
        simulated_random_number,
    );

    fn_vs_closure_comparison();

    another_use_for_closures();
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today!  Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn generate_workout_refactored_variable(intensity: u32, random_number: u32) {
    // We extract the duplicated call to become a variable.  This makes us only
    // call the function once, but we always call it, even if not needed.
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today!  Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

fn generate_workout_refactored_closure(intensity: u32, random_number: u32) {
    // This is a closure.  If there are no parameters used for the closure, it's
    // ||, and if more than one is used, it's |param1, param2|.
    // We use 'let' because it makes expensive_closure contain the definition of
    // an anonymous function, not the resulting value of calling the anonymous
    // function.
    // The type annotations are not required on closures, but can be done.
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!  Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// This struct will hold the closure and the resulting value.  The struct will
// be able to call the closureonly if we need the resulting value, and will
// cache that value for reuse.  This pattern is called memoization or lazy
// evaluation.
// For a struct to hold a closure, the closure must have its type defined.
// Each closure instance has its own unique anonymous type, even if they have
// the same signature.  Therefore, we use generics and trait bounds
struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    calculation: T,
    map: HashMap<U, Option<V>>,
}

// We want to have Cacher manage the struct's fields rather than letting the
// calling code potentially chage values directly, so we leave the fields are
// left private.
impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: std::cmp::Eq + std::hash::Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, key: U) -> V {
        match self.map.get(&key) {
            Some(val) => val.unwrap(),
            None => {
                let val = (self.calculation)(key);
                self.map.insert(key, Some(val));
                val
            }
        }
    }
}

fn generate_workout_refactored_closure_w_struct(intensity: u32, random_number: u32) {
    // This creates the closure within the struct, allowing us to only call it
    // once if needed, and then storing the result to be reused as needed.
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!  Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn fn_vs_closure_comparison() {
    // Below is a function definition and then several closures with varying
    // levels of type annotation to show the similarity between the two.
    // The last two examples need to be called in order to work because they
    // rely on type inference
    fn _add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let add_one_v3 = |x|             { x + 1 };
    //let add_one_v4 = |x|               x + 1  ;
}

fn another_use_for_closures() {
    // A closure can also capture the environment and access variables from the
    // scope in which they're defined.
    // Closures can capture values from their environment in three ways:
    // 1- FnOnce: Consumes the variable it captures from its enclosing scope
    // (known as the closure's environment).  To do this, the closure must take
    // ownership and move them into the closure when it is defined.  This move
    // means that it can only be done once.
    // 2- FnMut: Can change the environment because it mutably borrows values.
    // 3- Fn: Borrows values from the environment immutably.
    let x = 25;
    // If you want to force the closure to take ownership of the values it uses
    // in the environment, use 'move'.  They may still implement Fn or FnMut,
    // even though they capture variables by move.  This is because the traits
    // implemented by the closure types are only determined by what the closuer
    // does with the captured values, not how it captures them.  'move' only
    // specifies how it captures them.
    // let equal_to_x = move |z| z == x;
    let equal_to_x = |z| z == x;

    println!("{}", x);

    let y = 25;

    assert!(equal_to_x(y));
}

#[cfg(test)]
mod tests {
    use super::Cacher;
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_string_usize() {
        let mut c = Cacher::new(|a: &str| a.len());

        let v1 = c.value("Three");

        assert_eq!(v1, 5);
    }
}
