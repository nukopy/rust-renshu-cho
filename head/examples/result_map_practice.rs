fn main() {
    println!("Hello");
}

#[cfg(test)]
mod tests {
    #[test]
    fn result_map_practice() {
        let line = "1\n2\n3\n4\n";
        #[allow(clippy::identity_op)]
        let expected = 1 * 2 + 2 * 2 + 3 * 2 + 4 * 2;

        let mut count = 0;
        for num in line.lines() {
            #[allow(clippy::single_match)]
            match num.parse::<i32>().map(|i| i * 2) {
                Ok(n) => {
                    count += n;
                    println!("{n}");
                }
                Err(..) => {}
            }
        }

        assert_eq!(count, expected)
    }

    #[test]
    fn result_map_or_practice() {
        let calc_len = |v: &str| v.len();

        // if x is Ok, map returns applied function
        let x: Result<_, &str> = Ok("foo");
        #[allow(clippy::redundant_closure)]
        let result = x.map_or(42, |v| calc_len(v));
        assert_eq!(result, 3);

        // if x is Err, map returns default
        let x: Result<&str, _> = Err("bar");
        #[allow(clippy::redundant_closure)]
        let result = x.map_or(42, |v| calc_len(v));
        assert_eq!(result, 42);
    }

    #[test]
    fn result_map_or_else_practice() {
        // constants
        let k = 21;

        // closures
        let calc_len = |v: &str| v.len();
        #[allow(unused_variables)]
        let fallback = |e: &str| k * 2;

        // if x is Ok, applied fallback
        let x: Result<_, &str> = Ok("foo");
        #[allow(clippy::redundant_closure)]
        let result = x.map_or_else(|e| fallback(e), |v| calc_len(v));
        assert_eq!(result, 3);

        let x: Result<&str, _> = Err("bar");
        #[allow(clippy::redundant_closure)]
        let result = x.map_or_else(|e| fallback(e), |v| calc_len(v));
        assert_eq!(result, 42);
    }

    type MyResult = Result<i32, String>;
    fn divide(a: i32, b: i32) -> MyResult {
        if b == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    #[test]
    fn result_map_or_else_practice2() {
        // closure
        let double_and_convert_str = |v: i32| (v * 2).to_string();
        let fallback = |e: String| e.to_uppercase();

        // test divided by not zero
        let expected = 10.to_string();
        let result = divide(10, 2); // Ok(5)
        let result =
            result.map_or_else(|e: String| fallback(e), |v: i32| double_and_convert_str(v));
        assert_eq!(result, expected);

        // test divided by zero
        let expected = String::from("Division by zero").to_uppercase();
        let result = divide(10, 0); // Err(String("Division by zero"))
        let result =
            result.map_or_else(|e: String| fallback(e), |v: i32| double_and_convert_str(v));
        assert_eq!(result, expected);
    }

    /// 3 の倍数ならエラーコード 333、それ以外ならエラーコード 0 を返す
    fn get_error_code(code: u32) -> Result<u32, u32> {
        if code % 3 == 0 {
            Ok(333)
        } else {
            Err(code)
        }
    }

    /// map_err にて、値が Err(u32) のときに適用する関数
    fn stringify(x: u32) -> String {
        format!("error code: {x}")
    }

    #[test]
    #[allow(clippy::redundant_closure)]
    fn result_map_err() {
        // x は Ok(u32) なので、map_err はそのまま Ok(u32) を返す
        let x: Result<u32, u32> = get_error_code(3); // Ok(333)
        let expected: Result<u32, String> = Ok(333);
        let result: Result<u32, String> = x.map_err(|v: u32| stringify(v));
        assert_eq!(result, expected);

        // x は Err(u32) なので、map_err は Ok(String) 型を返す
        let x: Result<u32, u32> = get_error_code(2); // Err(2)
        let expected: Result<u32, String> = Err("error code: 2".to_string());
        let result: Result<u32, String> = x.map_err(|v: u32| stringify(v)); // Ok("error code: 2")
        assert_eq!(result, expected);
    }
}
