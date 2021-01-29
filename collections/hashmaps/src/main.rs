use std::collections::HashMap;

fn main() {
    let mut fruits = HashMap::new();

    fruits.insert("apple", 3);
    fruits.insert("mango", 6);
    fruits.insert("orange", 2);
    fruits.insert("avocado", 7);

    for (k, v) in &fruits {
        println!("I got {} {}", k, v);
    }

    fruits.remove("apple");

    let old_avocado = fruits["avocado"];

    fruits.insert("avocado", old_avocado + 2);

    println!("{}", fruits["avocado"]);
}
