pub fn times_two(num: i32) -> i32 {
    num * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_twice_of_positive_numbers() {
        // Arrange
        let input = 4;

        // Act
        let result = times_two(input);

        // Assert
        assert_eq!(result, 8);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        // Arrange
        let input = -4;

        // Act
        let result = times_two(input);

        // Assert
        assert_eq!(result, -8);
    }
}
