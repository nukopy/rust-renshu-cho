#[allow(dead_code)]
fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_returns_hello_world() {
        // given
        let name = "world";

        // when
        // 親モジュールの private な関数をテストする
        let result = greeting(name);

        // then
        // assert でエラーが発生した場合、エラーメッセージに name と result を表示するようなカスタムメッセージを表示することができる
        assert!(
            result.contains(name),
            "Greeting message does not contain the name: name={}, result={}",
            name,
            result
        );
        // assert_eq!(result, "Hello, world!");
    }
}
