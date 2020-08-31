pub fn get_fizzbuzz(num: i32) -> String {
    if num == 0 {
        String::from("0")
    } else if num % 15 == 0 {
        String::from("fizzbuzz")
    } else if num % 3 == 0 {
        String::from("fizz")
    } else if num % 5 == 0 {
        String::from("buzz")
    } else {
        num.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fizzbuzz() {
        assert_eq!(get_fizzbuzz(0), "0");
        assert_eq!(get_fizzbuzz(1), "1");
        assert_eq!(get_fizzbuzz(3), "fizz");
        assert_eq!(get_fizzbuzz(5), "buzz");
        assert_eq!(get_fizzbuzz(15), "fizzbuzz");
    }
}
