use anyhow::Result;

use test_util::{
    constants::uniq::SKIP,
    run::uniq::{
        run, run_count, run_outfile, run_outfile_count, run_stdin, run_stdin_count,
        run_stdin_outfile_count,
    },
};

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
