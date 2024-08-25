mod utils;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

use utils::constants::BINARY_NAME;

// --------------------------------------------------
// Test cases for general CLI flags
// --------------------------------------------------

/// commands to test:
///
/// with cargo:
///
/// - `cargo run -- -h`
/// - `cargo run -- --help`
///
/// with bin:
///
/// - `cat -h`
/// - `cat --help`
#[test]
fn usage() -> Result<()> {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(BINARY_NAME)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

/// commands to test:
///
/// with cargo:
///
/// - `cargo run -- -V`
/// - `cargo run -- --version`
///
/// with bin:
///
/// - `cat -V`
/// - `cat --version`
#[test]
fn version() -> Result<()> {
    for flag in &["-V", "--version"] {
        Command::cargo_bin(BINARY_NAME)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    }
    Ok(())
}
