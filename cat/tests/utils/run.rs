#![allow(dead_code)]

use anyhow::Result;
use assert_cmd::Command;
use pretty_assertions::assert_eq;
use std::fs;

pub fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(crate::BINARY_NAME)?
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

pub fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> Result<()> {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(crate::BINARY_NAME)?
        .write_stdin(input)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}
