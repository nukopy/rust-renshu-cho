mod utils;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

use utils::{
    constants::{BINARY_NAME, EMPTY, ONE},
    file::gen_bad_file,
    random::random_string,
};

#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(BINARY_NAME)?
        .args([EMPTY, &bad, ONE])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}
