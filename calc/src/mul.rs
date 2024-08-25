#![allow(dead_code)]
fn mul(left: u64, right: u64) -> u64 {
    left * right
}

pub mod siblings {
    pub fn mul(left: u64, right: u64) -> u64 {
        left * right
    }

    #[allow(dead_code)]
    fn mul_private(left: u64, right: u64) -> u64 {
        left * right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_2_and_2_equals_4() {
        // 親モジュールの非公開関数は呼び出すことができる
        let result = mul(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn mul_2_and_2_equals_4_siblings_public() {
        // 兄弟モジュールの公開関数は呼び出すことができる
        let result = siblings::mul(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn mul_2_and_2_equals_4_siblings_private() {
        // 親モジュールと異なり、兄弟モジュールは独立した名前空間を持つため、
        // 兄弟モジュールの非公開関数を呼び出すことはできない
        /*
        ✗  cargo test
        Compiling calc v0.1.0 (/Users/nukopy/Projects/Rust/rust-renshu-cho/calc)
        error[E0603]: function `mul_private` is private
        --> src/mul.rs:34:39
        |
        34 | ...ings::mul_private(2, 2);
        |          ^^^^^^^^^^^ private function
        |
        note: the function `mul_private` is defined here
        --> src/mul.rs:11:5
        |
        11 |     fn mul_private(left: u64, right: u64) -> u6...
        |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        */
        // let result = super::siblings::mul_private(2, 2);
        // assert_eq!(result, 4);
    }
}
