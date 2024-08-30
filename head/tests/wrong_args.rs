mod utils;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

use utils::{
    constants::{BINARY_NAME, EMPTY},
    random::random_string,
};

#[test]
fn dies_bad_bytes() -> Result<()> {
    let bad = random_string();
    let expected = format!(
        "invalid value '{bad}' for \
        '--bytes <BYTES>': invalid digit found in string"
    );

    Command::cargo_bin(BINARY_NAME)?
        .args(["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bad_lines() -> Result<()> {
    let bad = random_string();
    let expected = format!(
        "error: invalid value '{bad}' for \
        '--lines <LINES>': invalid digit found in string"
    );
    Command::cargo_bin(BINARY_NAME)?
        .args(["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_no_args_bytes() -> Result<()> {
    let expected = "error: a value is required for \
        '--bytes <BYTES>' but none was supplied";
    Command::cargo_bin(BINARY_NAME)?
        .args(["-c"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_no_args_lines() -> Result<()> {
    let expected = "error: a value is required for \
        '--lines <LINES>' but none was supplied";
    Command::cargo_bin(BINARY_NAME)?
        .args(["-n"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bytes_and_lines() -> Result<()> {
    let msg = "the argument '--lines <LINES>' cannot be \
               used with '--bytes <BYTES>'";

    Command::cargo_bin(BINARY_NAME)?
        .args(["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}
