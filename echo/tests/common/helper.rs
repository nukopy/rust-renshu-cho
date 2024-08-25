use std::fs;

use assert_cmd::Command;

use super::{constants::BINARY_NAME, types::TestResult};

pub fn compare_bash_echo_output(args: &[&str], filename: &str) -> TestResult {
    // テスト項目: echo コマンドが、bash の echo コマンドと同じ出力を行う
    // given (前提条件):
    // read file of output bash echo command
    let expected = fs::read_to_string(filename)?;

    // define cmd
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    // when (操作):
    // cmd.assert() の中でコマンドを実行している

    // then (期待する結果):
    cmd.args(args).assert().success().stdout(expected);

    Ok(())
}
