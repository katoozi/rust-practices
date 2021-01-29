fn main() {
    let mut x = 1024;

    loop {
        if x < 0 {
            break;
        }

        println!("{} more runs to go", x);

        x -= 1;
    }

    let a: u32 = 10;
    let b: u32 = 4;

    let result = silly_sub(a, b);

    println!("{} - {} = {}", a, b, result);

    // while loop
    let mut x2 = 1000;
    while x2 > 0 {
        println!("{} more runs to go", x2);
        x2 -= 1;
    }

    // for loop
    for i in 1..10 {
        println!("{} without 10", i);
    }
    for i in 1..=10 {
        println!("{} with 10", i);
    }
}

fn silly_sub(a: u32, b: u32) -> u32 {
    let mut result = 0;

    'increment: loop {
        if result == a {
            let mut dec = b;
            loop {
                if dec == 0 {
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
