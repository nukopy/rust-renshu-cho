use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

use test_util::{
    constants::uniq::{BINARY_NAME, EMPTY, ONE, SKIP, T1, T2, T3, T4, T5, T6, THREE, TWO},
    file::gen_bad_file,
    run::uniq::{
        run, run_count, run_outfile, run_outfile_count, run_stdin, run_stdin_count,
        run_stdin_outfile_count,
    },
};

// --------------------------------------------------
// error
// --------------------------------------------------

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

// --------------------------------------------------
// empty
// --------------------------------------------------

#[test]
fn empty() -> Result<()> {
    run(&EMPTY)
}

#[test]
fn empty_count() -> Result<()> {
    run_count(&EMPTY)
}

#[test]
fn empty_stdin() -> Result<()> {
    run_stdin(&EMPTY)
}

#[test]
fn empty_stdin_count() -> Result<()> {
    run_stdin_count(&EMPTY)
}

#[test]
fn empty_outfile() -> Result<()> {
    run_outfile(&EMPTY)
}

#[test]
fn empty_outfile_count() -> Result<()> {
    run_outfile_count(&EMPTY)
}

#[test]
fn empty_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&EMPTY)
}

// --------------------------------------------------
// one
// --------------------------------------------------

#[test]
fn one() -> Result<()> {
    run(&ONE)
}

#[test]
fn one_count() -> Result<()> {
    run_count(&ONE)
}

#[test]
fn one_stdin() -> Result<()> {
    run_stdin(&ONE)
}

#[test]
fn one_stdin_count() -> Result<()> {
    run_stdin_count(&ONE)
}

#[test]
fn one_outfile() -> Result<()> {
    run_outfile(&ONE)
}

#[test]
fn one_outfile_count() -> Result<()> {
    run_outfile_count(&ONE)
}

#[test]
fn one_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&ONE)
}

// --------------------------------------------------
// two
// --------------------------------------------------

#[test]
fn two() -> Result<()> {
    run(&TWO)
}

#[test]
fn two_count() -> Result<()> {
    run_count(&TWO)
}

#[test]
fn two_stdin() -> Result<()> {
    run_stdin(&TWO)
}

#[test]
fn two_stdin_count() -> Result<()> {
    run_stdin_count(&TWO)
}

#[test]
fn two_outfile() -> Result<()> {
    run_outfile(&TWO)
}

#[test]
fn two_outfile_count() -> Result<()> {
    run_outfile_count(&TWO)
}

#[test]
fn two_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&TWO)
}

// --------------------------------------------------
// three
// --------------------------------------------------

#[test]
fn three() -> Result<()> {
    run(&THREE)
}

#[test]
fn three_count() -> Result<()> {
    run_count(&THREE)
}

#[test]
fn three_stdin() -> Result<()> {
    run_stdin(&THREE)
}

#[test]
fn three_stdin_count() -> Result<()> {
    run_stdin_count(&THREE)
}

#[test]
fn three_outfile() -> Result<()> {
    run_outfile(&THREE)
}

#[test]
fn three_outfile_count() -> Result<()> {
    run_outfile_count(&THREE)
}

#[test]
fn three_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&THREE)
}

// --------------------------------------------------
// skip
// --------------------------------------------------

#[test]
fn skip() -> Result<()> {
    run(&SKIP)
}

#[test]
fn skip_count() -> Result<()> {
    run_count(&SKIP)
}

#[test]
fn skip_stdin() -> Result<()> {
    run_stdin(&SKIP)
}

#[test]
fn skip_stdin_count() -> Result<()> {
    run_stdin_count(&SKIP)
}

#[test]
fn skip_outfile() -> Result<()> {
    run_outfile(&SKIP)
}

#[test]
fn skip_outfile_count() -> Result<()> {
    run_outfile_count(&SKIP)
}

#[test]
fn skip_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&SKIP)
}

// --------------------------------------------------
// t1
// --------------------------------------------------

#[test]
fn t1() -> Result<()> {
    run(&T1)
}

#[test]
fn t1_count() -> Result<()> {
    run_count(&T1)
}

#[test]
fn t1_stdin() -> Result<()> {
    run_stdin(&T1)
}

#[test]
fn t1_stdin_count() -> Result<()> {
    run_stdin_count(&T1)
}

#[test]
fn t1_outfile() -> Result<()> {
    run_outfile(&T1)
}

#[test]
fn t1_outfile_count() -> Result<()> {
    run_outfile_count(&T1)
}

#[test]
fn t1_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T1)
}

// --------------------------------------------------
// t2
// --------------------------------------------------

#[test]
fn t2() -> Result<()> {
    run(&T2)
}

#[test]
fn t2_count() -> Result<()> {
    run_count(&T2)
}

#[test]
fn t2_stdin() -> Result<()> {
    run_stdin(&T2)
}

#[test]
fn t2_stdin_count() -> Result<()> {
    run_stdin_count(&T2)
}

#[test]
fn t2_outfile() -> Result<()> {
    run_outfile(&T2)
}

#[test]
fn t2_outfile_count() -> Result<()> {
    run_outfile_count(&T2)
}

#[test]
fn t2_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T2)
}

// --------------------------------------------------
// t3
// --------------------------------------------------

#[test]
fn t3() -> Result<()> {
    run(&T3)
}

#[test]
fn t3_count() -> Result<()> {
    run_count(&T3)
}

#[test]
fn t3_stdin() -> Result<()> {
    run_stdin(&T3)
}

#[test]
fn t3_stdin_count() -> Result<()> {
    run_stdin_count(&T3)
}

#[test]
fn t3_outfile() -> Result<()> {
    run_outfile(&T3)
}

#[test]
fn t3_outfile_count() -> Result<()> {
    run_outfile_count(&T3)
}

#[test]
fn t3_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T3)
}

// --------------------------------------------------
// t4
// --------------------------------------------------

#[test]
fn t4() -> Result<()> {
    run(&T4)
}

#[test]
fn t4_count() -> Result<()> {
    run_count(&T4)
}

#[test]
fn t4_stdin() -> Result<()> {
    run_stdin(&T4)
}

#[test]
fn t4_stdin_count() -> Result<()> {
    run_stdin_count(&T4)
}

#[test]
fn t4_outfile() -> Result<()> {
    run_outfile(&T4)
}

#[test]
fn t4_outfile_count() -> Result<()> {
    run_outfile_count(&T4)
}

#[test]
fn t4_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T4)
}

// --------------------------------------------------
// t5
// --------------------------------------------------

#[test]
fn t5() -> Result<()> {
    run(&T5)
}

#[test]
fn t5_count() -> Result<()> {
    run_count(&T5)
}

#[test]
fn t5_stdin() -> Result<()> {
    run_stdin(&T5)
}

#[test]
fn t5_stdin_count() -> Result<()> {
    run_stdin_count(&T5)
}

#[test]
fn t5_outfile() -> Result<()> {
    run_outfile(&T5)
}

#[test]
fn t5_outfile_count() -> Result<()> {
    run_outfile_count(&T5)
}

#[test]
fn t5_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T5)
}

// --------------------------------------------------
// t6
// --------------------------------------------------

#[test]
fn t6() -> Result<()> {
    run(&T6)
}

#[test]
fn t6_count() -> Result<()> {
    run_count(&T6)
}

#[test]
fn t6_stdin() -> Result<()> {
    run_stdin(&T6)
}

#[test]
fn t6_stdin_count() -> Result<()> {
    run_stdin_count(&T6)
}

#[test]
fn t6_outfile() -> Result<()> {
    run_outfile(&T6)
}

#[test]
fn t6_outfile_count() -> Result<()> {
    run_outfile_count(&T6)
}

#[test]
fn t6_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T6)
}
