fn christmas_carol_lyrics() {
    let ranks = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let mut day = 1;
    println!("\nThe 12 Days of Christmas:");
    for rank in ranks.iter() {
        println! {"On the {} day of Christmas my true love gave to me", rank};
        if day >= 12 {
            println!("Twelve drummers drumming,");
        }
        if day >= 11 {
            println!("Eleven pipers piping,");
        }
        if day >= 10 {
            println!("Ten lords a leaping,");
        }
        if day >= 9 {
            println!("Nine ladies dancing,");
        }
        if day >= 8 {
            println!("Eight maids a milking,");
        }
        if day >= 7 {
            println!("Seven swans a swimming,");
        }
        if day >= 6 {
            println!("Six geese a laying,");
        }
        if day >= 5 {
            println!("Five gold rings,");
        }
        if day >= 4 {
            println!("Four calling birds,");
        }
        if day >= 3 {
            println!("Three French hens,");
        }
        if day >= 2 {
            println!("Two turtle doves, and");
        }
        if day >= 1 {
            println!("A partridge in a pear tree\n");
        }
        day += 1;
    }
}

fn nth_fibonacci(n: i32) -> i32 {
    let mut nfib = 1;
    let mut nfib_last = 0;
    for _ in (1..n).rev() {
        let nfib_temp = nfib;
        nfib += nfib_last;
        nfib_last = nfib_temp;
    }
    nfib
}

fn temp_conversion(temp: f32, unit: &str) -> f32 {
    let new_temp: f32;
    if unit == "Fahrenheit" {
        new_temp = (temp - 32.0) * 5.0 / 9.0;
    } else if unit == "Celsius" {
        new_temp = temp * 9.0 / 5.0 + 32.0;
    } else {
        new_temp = -999.99;
    }
    new_temp
}

fn main() {
    let temp: f32 = 45.8;
    let unit = "Fahrenheit";
    let new_temp = temp_conversion(temp, unit);
    println!("{} F = {} C", temp, new_temp);
    let temp: f32 = 26.3;
    let unit = "Celsius";
    let new_temp = temp_conversion(temp, unit);
    println!("{} C = {} F", temp, new_temp);
    let n: i32 = 7;
    let nfib = nth_fibonacci(n);
    println!("Value {} of the fibonacci sequence is {}", n, nfib);
    christmas_carol_lyrics();
}
