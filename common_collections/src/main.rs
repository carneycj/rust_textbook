fn main() {
    // This covers the three most commonly used data structures that are
    // collections.  Most data types can only represent one specific value, but
    // collections can contain multiple values.

    // Unlike the built-in array and tuple types, the data these collections
    // point to are stored in the heap, allowing for variable size at runtime.

    // It's important to consider the benefits/costs of using the heap while
    // choosing the best data structures

    // The three most commonly used collections:
    // - Vector: Allows you to store a variable number of values next to each
    //      other
    // - String: A collection of characters
    // - Hash Map: Allows you to associate a value with a particular key.  This
    //      is a more specific implementation of a more general structure called
    //      a map
    exploring_vectors();
    exploring_strings();
    exploring_hashmaps();
    projects();
}

fn exploring_vectors() {
    // This creates a new, empty vector that will hold i32 values
    // It's important to provide the type of data that it will hold
    let v: Vec<i32> = Vec::new();

    // The nice thing is that as shown by this scrap code I threw together, it
    // accepts your custom types and structs, not just the pre-made types.
    enum Test {
        StringTest1(String),
        StringTest2(u8),
    }
    struct T {
        thing: i32,
        object: f64,
        test: Test,
    }
    let mut v2: Vec<T> = Vec::new();
    let mut v3: Vec<Test> = Vec::new();

    let my_struct = T {
        thing: 5,
        object: 6.4,
        test: Test::StringTest1(String::from("my_string")),
    };
    v2.push(my_struct);
    v3.push(Test::StringTest2(3));

    // This is a macro that creates a new vector for you that holds the values
    // you provide
    let v = vec![1, 2, 3];

    // Since all the values you add are i32, you don't need to add the type
    // annotation
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let mut v2 = vec![1, 2, 3, 4, 5];
    // This method of accessing an index is good for if you want your code to
    // panic when accessing an invalid index.  For example if you want to make
    // sure that a bad index is never accessed.
    let third: &i32 = &v2[2];
    println!("The third element is: {}", third);
    // This method of accessing an index is good for if you want your code to
    // gracefully handle an invalid index.  For example if you expect it to
    // sometimes happen but want to keep running.
    match v2.get(2) {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element."),
    }

    // Ownership and borrowing rules still apply, so we have to be careful when
    // working with vectors.  This is an example trying to use a mutable
    // reference while using an immutable reference.
    let first: &i32 = &v2[0];
    // Adding the next line would prevent the code from compiling because of the
    // print statement referencing first as well.
    // v.push(6);
    println!("The first element is: {}", first);

    // Iterating over a vector:
    let mut v3 = vec![100, 32, 57];
    // The mut keyword in not required for this method
    for i in &v3 {
        println!("{}", i);
    }
    // The mut keyword is required for this method
    for i in &mut v3 {
        // The * is a dereference operator.  It creates a pointer
        *i += 50;
        println!("{}", i);
    }

    // We can also use enums to store multiple types in a vector:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    } // The vector 'row' goes out of scope here and is freed
} // The vector v goes out of scope at the end of the function and is freed

fn exploring_strings() {
    // Strings are UTF-8 encoded, so any characters can be used
    // Creating a string
    let mut s = String::new();
    let data = "initial content";
    let s = data.to_string();
    // You can also apply to_string() directly
    let s = "initial content".to_string();
    // This creates a string from a literal also
    let s = String::from("initial content");

    // Updating a string
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is: {}", s2);

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    println!("s2 is: {}", s2);

    // .push() only can add one character to a string
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 has been moved here and can no longer be used, &s2 is a reference and
    // can.  We do it like this because of how + is designed:
    // fn add(self, s: &s2) -> String {...}
    // In standard lib it's designed using generics, but you get the point
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // Using + takes ownership of s1
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    let s1 = String::from("tic");
    // Using format! doesn't take ownership of s1
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("{}", s1);

    // A String is a wrapper over a Vec<u8>
    let hello = String::from("Hola");
    println!("{}", hello.len());
    let hello = String::from("Здравствуйте");
    println!("{}", hello.len());
    // Note that in this case the length of the second one doesn't match the
    // number of characters in it.  This is because it uses UTF-8 encoding, so
    // one byte doesn't return one character.  It's important to also consider
    // that this means that the len() function provides the number of bytes,
    // not the number of characters.
    // This also means that you can't go digging for characters using normal
    // indexing like this.  It would only provide the [n]th byte of the string,
    // not the [n]th character of the string. like you'd want
    // let hello = "hello";
    // let answer = &hello[0];

    // Rust looks at strings in three ways: bytes, scalar values, and grapheme
    // clusters
    // We're going to consider “नमस्ते”:
    // Bytes: A list of u8 values to create each UTF-8 character - [224, 164,
    // 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224,
    // 165, 135]
    // Scalar Values: A list of UTF-8 characters - ['न', 'म', 'स', '्', 'त', 'े']
    // Grapheme Clusters: A list of "letters" - ["न", "म", "स्", "ते"]

    // You can index into a string, but it isn't recommended because it's hard
    // to tell if you're asking for bytes, scalars, or clusters
    let s = &hello[0..4];
    // You have to be careful too because this won't work:
    // let s = &hello[0..1];
    println!("{}", s);

    // But you can use some premade options:
    let hello = "नमस्ते";
    println!("Bytes:");
    for b in hello.bytes() {
        println!("{}", b);
    }
    println!("Characters:");
    for c in hello.chars() {
        println!("{}", c);
    }
    // Grapheme Clusters are harder to do, so not in the standard library
    // Use unicode-segmentation:
    use unicode_segmentation::UnicodeSegmentation;
    println!("Graphemes:");
    for g in hello.graphemes(true) {
        println!("{}", g);
    }
}

fn exploring_hashmaps() {
    // HashMap<K, V> stores a hash map of keys of type K to values of type V
    // use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Another way to create a hash map is by using iterators and collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // If the type used implements copy, then a copy is used.  Otherwise,
    // ownership is transferred
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // teams, field_name, and field_value are no longer usable.  initial_scores
    // is.  All are still accessible via the HashMap now though

    let team_name = String::from("Blue");
    // The result of get here is Some(&10) because it may be none
    let score = scores.get(&team_name);

    // The order will be arbitrary
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // While hash maps are growable, a key can only point to one value.  This
    // means that when adding information to your hash map, you have to decide
    // how to handle the case when a key already has a value assigned.  Replace
    // old with new, keep old and ignore new, or combine old with new
    // Overwriting:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only inserting a value if the key has no value:
    // It's better to use the premade functions because they play better with
    // the borrow checker
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(40);
    println!("{:?}", scores);

    // Updating a value based on the old value:
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns &mut v (value to the key) and we store it as count
        let count = map.entry(word).or_insert(0);
        // That's why we can dereference count and add 1 to the value
        *count += 1;
    }
    println!("{:?}", map);
}

fn projects() {
    // Get the mean, median, and mode from a list of values
    let mut my_vec: Vec<i32> = vec![1, 3, 3, 4, 6, 8, 9, 12, 15, 25, 25, 25, 30, 32];

    let my_mean: f32 = calc_mean(&my_vec);
    println!("mean: {}", my_mean);

    let my_median: f32 = calc_median(&mut my_vec);
    println!("median: {}", my_median);

    let my_mode: i32 = calc_mode(&my_vec);
    println!("mode: {}", my_mode);

    // Convert strings to pig-latin
    let string1 = String::from("popsicle");
    let string2 = String::from("apple");
    let string3 = String::from("Здравствуйте");
    println!("{} becomes {}", &string1, pig_latin(&string1));
    println!("{} becomes {}", &string2, pig_latin(&string2));
    println!("{} becomes {}", &string3, pig_latin(&string3));
    println!("{} becomes {}", &string1, prettier_pig_latin(&string1));
    println!("{} becomes {}", &string2, prettier_pig_latin(&string2));
    // println!("{} becomes {}", &string3, prettier_pig_latin(&string3));

    // Using hash maps and vectors, create a text interface to add employees to
    // a department ("Add Sally to Engineering", "Add Amir to Sales").  Then let
    // user retrieve a list of all people in a department or all people in the
    // company by department, sorted alphabetically
    emp_list();
}

fn calc_mean(my_vec: &Vec<i32>) -> f32 {
    (my_vec.iter().sum::<i32>() as f32) / (my_vec.len() as f32)
}

fn calc_median(my_vec: &mut Vec<i32>) -> f32 {
    let vec_len = my_vec.len();
    let midpoint = vec_len / 2;

    my_vec.sort();
    match vec_len % 2 {
        0 => ((my_vec[midpoint] as f32) + (my_vec[midpoint - 1] as f32)) / 2.0,
        _ => my_vec[midpoint] as f32,
    }
}

fn calc_mode(my_vec: &Vec<i32>) -> i32 {
    // use std::collections::HashMap;
    let mut num_counts = HashMap::new();
    for num in my_vec {
        let count = num_counts.entry(num).or_insert(0);
        *count += 1;
    }
    *num_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Can't calculate mode of list size 0")
}

fn pig_latin(word: &String) -> String {
    let mut root_word: String = String::new();
    let mut first_letter: char = word.chars().next().unwrap();
    let end = String::from("ay");
    let mut count = 0;
    for letter in word.chars() {
        if count == 0 {
            first_letter = match letter {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    root_word.push(letter);
                    'h'
                }
                _ => letter,
            };
        } else {
            root_word.push(letter);
        }
        count += 1;
    }
    format!("{}{}{}", root_word, first_letter, end)
}

fn prettier_pig_latin(word: &String) -> String {
    // This is definitely the nicer way to write it, but it's important to
    // remember that this won't work if trying to access non-ASCII letters.
    // Lucky for us, pig latin needs latin letters, and that's ASCII.
    let first_letter: char = word.chars().next().unwrap();
    match first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}hay", word.to_string()),
        // The next line's splice of word is what would fail if not working on
        // ASCII characters
        _ => format!("{}{}ay", word[1..].to_string(), first_letter),
    }
}

use std::collections::hash_map::Entry;
use std::collections::HashMap;
// BufRead is used to allow the for loop (.lines())
use itertools::Itertools;
use std::io::{self, BufRead};
fn emp_list() {
    // "Add Sally to Engineering";
    // "Add Joe to Engineering";
    // "Add Bob to Engineering";
    // "Add Chris to Analytics";
    // "Add Sue to Analytics";
    // "Add Amir to Sales";

    let mut org_chart: HashMap<String, Vec<String>> = HashMap::new();

    println!("Type 'Add <name> to <department>' to add an employee:");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");
    for line in io::stdin().lock().lines() {
        let input = line.expect("Error: Unable to read user input");
        match Command::parse_line(&input) {
            Command::Add { name, department } => match org_chart.entry(department.to_string()) {
                Entry::Vacant(department) => {
                    department.insert(vec![name]);
                }
                Entry::Occupied(mut department) => {
                    department.get_mut().push(name);
                }
            },
            Command::List { department } => match org_chart.get(&department) {
                Some(names) => {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", department, name);
                    }
                }
                None => {
                    println!("No one works in that department");
                }
            },
            Command::All => {
                println!("{:?}", org_chart.keys());
                for department in org_chart.keys().sorted() {
                    let mut names = org_chart.get(department).unwrap().clone();
                    names.sort();
                    for name in names {
                        println!("{}: {:?}", department, name);
                    }
                }
            }
            Command::Quit => break,
            Command::InputError => println!("Input error.  Please try again"),
        }
    }
}

enum Command {
    Add { name: String, department: String },
    List { department: String },
    All,
    Quit,
    InputError,
}

impl Command {
    fn parse_line(line: &str) -> Command {
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        match words.as_slice() {
            ["Add", name, "to", department] => Command::Add {
                name: name.to_string(),
                department: department.to_string(),
            },
            ["List", department] => Command::List {
                department: department.to_string(),
            },
            ["All"] => Command::All,
            ["Quit"] => Command::Quit,
            _ => Command::InputError,
        }
    }
}
