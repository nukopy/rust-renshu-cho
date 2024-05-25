use assert_cmd::Command;

#[test]
fn runs() {
    // テスト項目: hello が
    // given (前提条件):
    let binary_name = "hello";
    let expected_stdout = "Hello, world!\n";
    let cmd = Command::cargo_bin(binary_name);
    let mut cmd = match cmd {
        Ok(cmd) => cmd,
        Err(err) => panic!("{}", err),
    };

    // when (操作):
    let _res = cmd.output();

    // then (期待する結果):
    cmd.assert().success().stdout(expected_stdout);
}

#[test]
fn true_exits_with_exit_code_0() {
    // テスト項目: true コマンドの終了コードは 0 である
    // given (前提条件):
    let cmd = Command::cargo_bin("true");
    let mut cmd = match cmd {
        Ok(cmd) => cmd,
        Err(err) => panic!("{}", err),
    };

    // when (操作):
    let _res = cmd.output();

    // then (期待する結果):
    cmd.assert().success();
}

#[test]
fn false_exits_with_exit_code_1() {
    // テスト項目: false コマンドの終了コードは 1 である
    // given (前提条件):
    let cmd = Command::cargo_bin("false");
    let mut cmd = match cmd {
        Ok(cmd) => cmd,
        Err(err) => panic!("{}", err),
    };

    // when (操作):
    let _res = cmd.output();

    // then (期待する結果):
    cmd.assert().failure();
}
