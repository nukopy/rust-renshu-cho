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

    #[test]
    #[ignore = "テスト関数が無視されることを確認する"]
    fn mul_2_and_2_equals_4_ignored() {
        // 親モジュールの非公開関数は呼び出すことができる
        let result = mul(2, 2);
        assert_eq!(result, 4);
    }
}

/* #[ignore] 属性の付与でテスト関数が無視されることを確認する

```sh
$ cargo test
   Compiling calc v0.1.0 (/Users/nukopy/Projects/Rust/rust-renshu-cho/calc)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/calc-1543b18eb8606239)

running 6 tests
test mul::tests::mul_2_and_2_equals_4_ignored ... ignored, テスト関数が無視されることを確認する
test add::tests::add_2_and_2_equals_4 ... ok
test mul::tests::mul_2_and_2_equals_4_siblings_private ... ok
test mul::tests::mul_2_and_2_equals_4 ... ok
test mul::tests::mul_2_and_2_equals_4_siblings_public ... ok
test sub::tests::sub_2_and_2_equals_0 ... ok

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests calc

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

 */
