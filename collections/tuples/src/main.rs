fn main() {
    let num_and_str: (u32, &str) = (10, "test");
    println!("{:?}", num_and_str);

    let (num, string) = num_and_str;

    println!("{}, {}", num, string);
}
