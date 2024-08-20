mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::{constants::BINARY_NAME, helper, types::TestResult};

#[test]
fn dies_no_args() -> TestResult {
    // テスト項目: TEXT 引数が指定されていない場合、ヘルプの "Usage" を含む文字列の出力が行われる
    // given (前提条件):
    let expected_text = "Usage";
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    // when (操作):
    // cmd.assert() の中でコマンドを実行している

    // then (期待する結果):
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(expected_text));

    Ok(())
}

#[test]
fn output_hello_world() -> TestResult {
    // テスト項目: echo コマンドが "Hello, world!" という文字列を出力する
    // given (前提条件):
    let arg = "Hello, world!";
    let expected_stdout = "Hello, world!\n";
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    // when (操作):
    // cmd.assert() の中でコマンドを実行している

    // then (期待する結果):
    cmd.arg(arg).assert().success().stdout(expected_stdout);

    Ok(())
}

#[test]
fn compare_bash_echo_output_hello1() -> TestResult {
    // テスト項目: echo コマンドが、bash の echo コマンドと同じ出力を行う 1
    // given (前提条件):
    // read file of output bash echo command
    let filename = "tests/expected/hello1.txt";
    let args = vec!["Hello there"];

    // when (操作):
    // helper 関数内の cmd.assert() の中でコマンドを実行している

    // then (期待する結果):
    helper::compare_bash_echo_output(&args, filename)?;

    Ok(())
}

#[test]
fn compare_bash_echo_output_hello2() -> TestResult {
    // テスト項目: echo コマンドが、bash の echo コマンドと同じ出力を行う 2
    // given (前提条件):
    // read file of output bash echo command
    let filename = "tests/expected/hello2.txt";
    let args = vec!["Hello", "there"];

    // when (操作):
    // helper 関数内の cmd.assert() の中でコマンドを実行している

    // then (期待する結果):
    helper::compare_bash_echo_output(&args, filename)?;

    Ok(())
}

#[test]
fn compare_bash_echo_output_hello1_newline() -> TestResult {
    // テスト項目: echo コマンドが、bash の echo コマンドと同じ出力を行う 3
    // given (前提条件):
    // read file of output bash echo command
    let filename = "tests/expected/hello1.n.txt";
    let args = vec!["-n", "Hello  there"];

    // when (操作):
    // helper 関数内の cmd.assert() の中でコマンドを実行している

    // then (期待する結果):
    helper::compare_bash_echo_output(&args, filename)?;

    Ok(())
}

#[test]
fn compare_bash_echo_output_hello2_newline() -> TestResult {
    // テスト項目: echo コマンドが、bash の echo コマンドと同じ出力を行う 4
    // given (前提条件):
    // read file of output bash echo command
    let filename = "tests/expected/hello2.n.txt";
    let args = vec!["-n", "Hello", "there"];

    // when (操作):
    // helper 関数内の cmd.assert() の中でコマンドを実行している

    // then (期待する結果):
    helper::compare_bash_echo_output(&args, filename)?;

    Ok(())
}
