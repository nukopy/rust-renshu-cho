use anyhow::Result;

use test_util::{
    constants::uniq::TWO,
    run::uniq::{
        run, run_count, run_outfile, run_outfile_count, run_stdin, run_stdin_count,
        run_stdin_outfile_count,
    },
};

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
