pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_should_return_four_when_given_two_and_two() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
