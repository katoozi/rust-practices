#[cfg(test)]
mod tests {
    use super::pow;

    #[test]
    fn test_pow() {
        assert_eq!(pow(-2, 3), -8);
    }
}

pub fn pow(base: i64, exponent: usize) -> i64 {
    let mut res: i64 = 1;

    if exponent == 0 {
        return 1;
    }

    for _ in 0..exponent {
        res *= base;
    }

    return res;
}
