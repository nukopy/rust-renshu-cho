pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_2_and_2_equals_0() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }
}
