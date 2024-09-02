use anyhow::Result;

use test_util::{
    constants::uniq::ONE,
    run::uniq::{
        run, run_count, run_outfile, run_outfile_count, run_stdin, run_stdin_count,
        run_stdin_outfile_count,
    },
};

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
