use anyhow::Result;

use test_util::{
    constants::uniq::THREE,
    run::uniq::{
        run, run_count, run_outfile, run_outfile_count, run_stdin, run_stdin_count,
        run_stdin_outfile_count,
    },
};

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
