use super::error::OVERFLOW_ERROR;

pub fn add8(a: i8, b: i8) -> Result<i8, &'static str> {
    a.checked_add(b).ok_or(OVERFLOW_ERROR)
}

pub fn add32(a: i32, b: i32) -> Result<i32, &'static str> {
    a.checked_add(b).ok_or(OVERFLOW_ERROR)
}

pub fn add64(a: i64, b: i64) -> Result<i64, &'static str> {
    a.checked_add(b).ok_or(OVERFLOW_ERROR)
}


// Runs test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add8(2, 2);
        assert_eq!(result, Ok(4));
    }

    #[test]
    fn test_overflow() {
        let result = add32(i32::MAX, 1);
        assert_eq!(result, Err(OVERFLOW_ERROR)); // Expect an error
    }
}