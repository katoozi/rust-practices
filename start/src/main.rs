use std::env;

fn main() {
    let name = env::args().skip(1).next();

    match name {
        Some(n) => println!("Hi there {} !", n),
        None => panic!("Dont receive any name"),
    }

    let mut target = "target";

    let mut greetings = "greet";

    println!("{}, {}", target, greetings);

    greetings = "number2";

    target = "asdasdasd";

    println!("{}, {}", target, greetings);

    let a: u64 = 17;

    let b = 3;

    let result = add(a, b);

    println!("Result {}", result);

    // is rust if is not statement it's a expression
    if result == 30 {
        println!("true");
    } else {
        println!("false");
    }

    let final_value = if result != 30 { "true" } else { "false" };

    println!("{}", final_value);

    let final_value = if result != 30 {
        "true";
    } else {
        "false";
    };

    println!("value {:?}", final_value);
}

// will sum up the a and b
fn add(mut a: u64, b: u64) -> u64 {
    // return keyword is optional
    a += 10;
    return a + b; // or a + b
}

fn silly_sub(a: i32, b: i32) -> i32 {
    let mut result = 0;
    'increment: loop {
        if result == a {
            let mut dec = b;
            loop {
                if dec == 0 {
                    // breaks directly out of 'increment loop
                    break 'increment;
                } else {
                    result -= 1;
                    dec -= 1;
                }
            }
        } else {
            result += 1;
        }
    }
    result
}
