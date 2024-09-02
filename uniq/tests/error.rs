use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

use test_util::{constants::uniq::BINARY_NAME, file::gen_bad_file};

#[test]
fn dies_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(BINARY_NAME)?
        .arg(bad)
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}
