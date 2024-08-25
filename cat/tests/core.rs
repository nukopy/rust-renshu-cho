mod utils;

use anyhow::Result;

use utils::{
    constants::{BINARY_NAME, BUSTLE, EMPTY, FOX, SPIDERS},
    run::{run, run_stdin},
};

// --------------------------------------------------
// Test cases for stdin
// --------------------------------------------------

#[test]
fn bustle_stdin() -> Result<()> {
    run_stdin(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")
}

#[test]
fn bustle_stdin_n() -> Result<()> {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected/the-bustle.txt.n.stdin.out",
    )
}

#[test]
fn bustle_stdin_b() -> Result<()> {
    run_stdin(
        BUSTLE,
        &["-b", "-"],
        "tests/expected/the-bustle.txt.b.stdin.out",
    )
}

// --------------------------------------------------
// Test cases for files input
// --------------------------------------------------

#[test]
fn empty() -> Result<()> {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n() -> Result<()> {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

#[test]
fn empty_b() -> Result<()> {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

#[test]
fn fox() -> Result<()> {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_n() -> Result<()> {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

#[test]
fn fox_b() -> Result<()> {
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

#[test]
fn spiders() -> Result<()> {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn spiders_n() -> Result<()> {
    run(&["--number", SPIDERS], "tests/expected/spiders.txt.n.out")
}

#[test]
fn spiders_b() -> Result<()> {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

#[test]
fn bustle() -> Result<()> {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn bustle_n() -> Result<()> {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

#[test]
fn bustle_b() -> Result<()> {
    run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

#[test]
fn all() -> Result<()> {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

#[test]
fn all_n() -> Result<()> {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

#[test]
fn all_b() -> Result<()> {
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
