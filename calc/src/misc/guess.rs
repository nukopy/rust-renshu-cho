#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        // if value < 1 || value > 100 {
        let guess_range = 1..=100;
        if !guess_range.contains(&value) {
            panic!(
                "予想値は 1 から 100 の間でなければなりません: value={}",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "予想値は 1 から 100 の間でなければなりません: value=0")]
    fn new_0_occur_panic() {
        // given
        // when: 境界条件
        Guess::new(0);
    }

    #[test]
    fn new_1_is_ok() {
        // given
        // when
        let guess = Guess::new(1);

        // then
        assert_eq!(guess.value, 1);
    }

    #[test]
    fn new_42_is_ok() {
        // given
        // when
        let guess = Guess::new(42);

        // then
        assert_eq!(guess.value, 42);
    }

    #[test]
    fn new_100_is_ok() {
        // given
        // when: 境界条件
        let guess = Guess::new(100);

        // then
        assert_eq!(guess.value, 100);
    }

    #[test]
    #[should_panic(expected = "予想値は 1 から 100 の間でなければなりません: value=101")]
    fn new_101_occur_panic() {
        // given
        Guess::new(101);
    }
}
