fn closure() {
    let doubler = |x| x * 2;

    let value = 2;

    let twice = doubler(value);

    println!("{} doubled is {}", value, twice);

    let big_clouser = |a, b| {
        let z = a + b;
        return z;
    };

    println!("result from clouser {}", big_clouser(5, 7));
}
