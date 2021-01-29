fn main() {
    let mut numbers_vec: Vec<u8> = Vec::new();

    numbers_vec.push(10);
    numbers_vec.push(11);
    numbers_vec.push(12);
    numbers_vec.push(13);

    let mut vec_with_macro = vec![1, 2];

    vec_with_macro.push(20);

    let _ = vec_with_macro.pop(); // drop the value by _

    let message = if numbers_vec == vec_with_macro {
        "they are equal"
    } else {
        "not equal"
    };

    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro);
}
