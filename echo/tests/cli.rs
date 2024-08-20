use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    // テスト項目: TEXT 引数が指定されていない場合、ヘルプの "Usage" を含む文字列の出力が行われる
    // given (前提条件):
    let binary_name = "echo";
    let expected_text = "Usage";
    let cmd = Command::cargo_bin(binary_name);
    let mut cmd = match cmd {
        Ok(cmd) => cmd,
        Err(err) => panic!("{}", err),
    };

    // when (操作):
    let _res = cmd.output();

    // then (期待する結果):
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(expected_text));
}

#[test]
fn output_hello_world() {
    // テスト項目: echo コマンドが "Hello, world!" という文字列を出力する
    // given (前提条件):
    let binary_name = "echo";
    let expected_stdout = "Hello, world!\n";
    let cmd = Command::cargo_bin(binary_name);
    let mut cmd = match cmd {
        Ok(cmd) => cmd,
        Err(err) => panic!("{}", err),
    };

    // when (操作):
    let _res = cmd.arg("Hello, world!").output();

    // then (期待する結果):
    cmd.assert().success().stdout(expected_stdout);
}
