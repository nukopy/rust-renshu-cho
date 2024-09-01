/// 正の整数をパースする（TDD の練習）
pub fn parse_positoive_int(val: &str) -> Result<usize, Box<dyn std::error::Error>> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}
pub fn parse_positoive_int_2(val: &str) -> Result<usize, Box<dyn std::error::Error>> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests_parse {
    use super::parse_positoive_int;

    #[test]
    fn test_parse_positive_int_returns_int() {
        // テスト項目: 正の整数 3 を正しくパースする
        // given (前提条件):
        let target_str = "3";
        let expected = 3;

        // when (操作):
        let result = parse_positoive_int(target_str);

        // then (期待する結果):
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_not_digit_returns_error() {
        // テスト項目: 数字でない文字列をパースした場合、エラーを返す
        // given (前提条件):
        let target_str = "foo";

        // when (操作):
        let result = parse_positoive_int(target_str);

        // then (期待する結果):
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), target_str.to_string());
    }

    #[test]
    fn test_parse_zero_returns_error() {
        // テスト項目: 数字でない文字列をパースした場合、エラーを返す
        // given (前提条件):
        let target_str = "0";

        // when (操作):
        let result = parse_positoive_int(target_str);

        // then (期待する結果):
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), target_str.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::str::FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "hoge" {
            Ok(Point { x: 333, y: 1234 })
        } else {
            Err("not hoge")
        }
    }
}

#[cfg(test)]
mod tests_from_str {
    use std::str::FromStr;

    use super::Point;

    // ----------------------------------------
    // std::str::FromStr
    // ----------------------------------------

    #[test]
    fn test_point_from_str_hoge_returns_x333_y1234_point() {
        // テスト項目:
        // given (前提条件): Point::parse で "hoge" をパースした時、x=333, y=1234 の Point 構造体を取得できる
        let target_str = "hoge";

        // when (操作):
        let result = Point::from_str(target_str);
        let p = result.unwrap();

        // then (期待する結果):
        assert_eq!(p.x, 333);
        assert_eq!(p.y, 1234);
    }

    #[test]
    fn test_point_from_str_not_hoge_returns_err() {
        // テスト項目:
        // given (前提条件): Point::parse で "hoge" をパースした時、x=333, y=1234 の Point 構造体を取得できる
        let target_str = "hogehogehoge";

        // when (操作):
        let result = Point::from_str(target_str);

        // then (期待する結果):
        assert_eq!(result.unwrap_err().to_string(), "not hoge");
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests_try_from {
    use super::*;

    #[test]
    fn test_even_number_try_from_8_returns_ok() {
        // テスト項目:
        // given (前提条件):
        let target_num = 8;
        let expected = Ok(EvenNumber(target_num));

        // when (操作):
        let result = EvenNumber::try_from(target_num);

        // then (期待する結果):
        assert_eq!(result, expected);
    }

    #[test]
    fn test_even_number_try_from_5_returns_err() {
        // テスト項目:
        // given (前提条件):
        let target_num = 5;
        let expected = Err(());

        // when (操作):
        let result = EvenNumber::try_from(target_num);

        // then (期待する結果):
        assert_eq!(result, expected);
    }
}
