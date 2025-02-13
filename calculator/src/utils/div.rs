
#[allow(dead_code)]
pub fn div8(a: i8, b: i8) -> Result<i8, &'static str> {
    if b == 0 {
        return Err("Division by zero");
    }
    a.checked_div(b).ok_or("Overflow")
}

// Runs test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = div8(2, 2);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn test_div_zero() {
        let result = div8(i8::MAX, 0);
        assert_eq!(result, Err("Division by zero")); // Expect an error
    }
}