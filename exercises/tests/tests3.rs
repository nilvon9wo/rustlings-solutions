pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // arrange
        let x = 4;

        // act
        let result = is_even(x);

        // assert
        assert!(result);
    }

    #[test]
    fn is_false_when_odd() {
        // arrange
        let x = 3;

        // act
        let result = is_even(x);

        // assert
        assert!(!result);
    }
}
