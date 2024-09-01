use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

use test_util::{constants::wc::BINARY_NAME, file::gen_bad_file};

// --------------------------------------------------
#[test]
fn dies_chars_and_bytes() -> Result<()> {
    Command::cargo_bin(BINARY_NAME)?
        .args(["-m", "-c"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "the argument '--chars' cannot be used with '--bytes'",
        ));
    Ok(())
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(BINARY_NAME)?
        .arg(bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}
