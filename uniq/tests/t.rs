use anyhow::Result;

use test_util::{
    constants::uniq::{T1, T2, T3, T4, T5, T6},
    run::uniq::{
        run, run_count, run_outfile, run_outfile_count, run_stdin, run_stdin_count,
        run_stdin_outfile_count,
    },
};

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
