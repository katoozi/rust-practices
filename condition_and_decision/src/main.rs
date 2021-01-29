fn main() {
    let rust_is_awesome = true;
    if rust_is_awesome {
        println!("indeed");
    } else {
        println!("well, you should try rust");
    }

    let result = if 1 == 2 {
        "wait what?"
    } else {
        "Rust makes sense"
    };
    println!("you know what? {}.", result);

    // throw the value away by adding semicolon
    let result2 = if 1 == 2 {
        "Nothing make sens";
    } else {
        "blah blah";
    };
    println!("result of computation: {:?}", result2);
}
