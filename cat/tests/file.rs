mod common;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

use common::{constants::BINARY_NAME, file::gen_bad_file};

#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(BINARY_NAME)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}
