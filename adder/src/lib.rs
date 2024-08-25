fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod siblings {
    pub fn sub(left: u64, right: u64) -> u64 {
        left - right
    }

    fn mul(left: u64, right: u64) -> u64 {
        left * right
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_2_and_2_equals_4() {
        let result = super::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn sub_2_and_2_equals_0() {
        let result = super::siblings::sub(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn mul_2_and_2_equals_4() {
        let result = super::siblings::mul(2, 2);
        assert_eq!(result, 4);
    }
}
