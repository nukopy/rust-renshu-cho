pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_2_and_2_equals_4() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
