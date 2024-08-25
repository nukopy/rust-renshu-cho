mod utils;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

use utils::{
    constants::BINARY_NAME,
    file::{
        cleanup, gen_chmod_000_file, gen_not_exist_file, setup, DIRNAME_CHMOD_000,
        DIRNAME_NOT_EXIST,
    },
};

#[test]
fn skips_non_existent_file() -> Result<()> {
    setup(DIRNAME_NOT_EXIST)?;

    // error message: No such file or directory (os error 2)
    let filename = gen_not_exist_file(DIRNAME_NOT_EXIST);
    let expected = format!("{filename}: .* [(]os error 2[)]");
    Command::cargo_bin(BINARY_NAME)?
        .arg(&filename)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    cleanup(DIRNAME_NOT_EXIST)?;

    Ok(())
}

#[test]
fn skips_chmod_000_file() -> Result<()> {
    setup(DIRNAME_CHMOD_000)?;

    // error message: Failed to open tmp.txt: Permission denied (os error 13)
    let filename = gen_chmod_000_file(DIRNAME_CHMOD_000)?;
    let expected = format!("{filename}: .* [(]os error 13[)]");
    Command::cargo_bin(BINARY_NAME)?
        .arg(&filename)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    // cleanup
    cleanup(DIRNAME_CHMOD_000)?;

    Ok(())
}
