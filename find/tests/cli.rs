use anyhow::Result;

use test_util::run::find::run;

#[test]
fn path1() -> Result<()> {
    run(&["tests/inputs"], "tests/expected/path1.txt")
}

#[test]
fn path_a() -> Result<()> {
    run(&["tests/inputs/a"], "tests/expected/path_a.txt")
}

#[test]
fn path_a_b() -> Result<()> {
    run(&["tests/inputs/a/b"], "tests/expected/path_a_b.txt")
}

#[test]
fn path_d() -> Result<()> {
    run(&["tests/inputs/d"], "tests/expected/path_d.txt")
}

#[test]
fn path_a_b_d() -> Result<()> {
    run(
        &["tests/inputs/a/b", "tests/inputs/d"],
        "tests/expected/path_a_b_d.txt",
    )
}

#[test]
fn type_f() -> Result<()> {
    run(&["tests/inputs", "-t", "f"], "tests/expected/type_f.txt")
}

#[test]
fn type_f_path_a() -> Result<()> {
    run(
        &["tests/inputs/a", "-t", "f"],
        "tests/expected/type_f_path_a.txt",
    )
}

#[test]
fn type_f_path_a_b() -> Result<()> {
    run(
        &["tests/inputs/a/b", "--type", "f"],
        "tests/expected/type_f_path_a_b.txt",
    )
}

#[test]
fn type_f_path_d() -> Result<()> {
    run(
        &["tests/inputs/d", "--type", "f"],
        "tests/expected/type_f_path_d.txt",
    )
}

#[test]
fn type_f_path_a_b_d() -> Result<()> {
    run(
        &["tests/inputs/a/b", "tests/inputs/d", "--type", "f"],
        "tests/expected/type_f_path_a_b_d.txt",
    )
}

#[test]
fn type_d() -> Result<()> {
    run(&["tests/inputs", "-t", "d"], "tests/expected/type_d.txt")
}

#[test]
fn type_d_path_a() -> Result<()> {
    run(
        &["tests/inputs/a", "-t", "d"],
        "tests/expected/type_d_path_a.txt",
    )
}

#[test]
fn type_d_path_a_b() -> Result<()> {
    run(
        &["tests/inputs/a/b", "--type", "d"],
        "tests/expected/type_d_path_a_b.txt",
    )
}

#[test]
fn type_d_path_d() -> Result<()> {
    run(
        &["tests/inputs/d", "--type", "d"],
        "tests/expected/type_d_path_d.txt",
    )
}

#[test]
fn type_d_path_a_b_d() -> Result<()> {
    run(
        &["tests/inputs/a/b", "tests/inputs/d", "--type", "d"],
        "tests/expected/type_d_path_a_b_d.txt",
    )
}

#[test]
fn type_l() -> Result<()> {
    run(&["tests/inputs", "-t", "l"], "tests/expected/type_l.txt")
}

#[test]
fn type_f_l() -> Result<()> {
    run(
        &["tests/inputs", "-t", "l", "f"],
        "tests/expected/type_f_l.txt",
    )
}

#[test]
fn name_csv() -> Result<()> {
    run(
        &["tests/inputs", "-n", ".*[.]csv"],
        "tests/expected/name_csv.txt",
    )
}

#[test]
fn name_csv_mp3() -> Result<()> {
    run(
        &["tests/inputs", "-n", ".*[.]csv", "-n", ".*[.]mp3"],
        "tests/expected/name_csv_mp3.txt",
    )
}

#[test]
fn name_txt_path_a_d() -> Result<()> {
    run(
        &["tests/inputs/a", "tests/inputs/d", "--name", ".*.txt"],
        "tests/expected/name_txt_path_a_d.txt",
    )
}

#[test]
fn name_a() -> Result<()> {
    run(&["tests/inputs", "-n", "a"], "tests/expected/name_a.txt")
}

#[test]
fn type_f_name_a() -> Result<()> {
    run(
        &["tests/inputs", "-t", "f", "-n", "a"],
        "tests/expected/type_f_name_a.txt",
    )
}

#[test]
fn type_d_name_a() -> Result<()> {
    run(
        &["tests/inputs", "--type", "d", "--name", "a"],
        "tests/expected/type_d_name_a.txt",
    )
}

#[test]
fn path_g() -> Result<()> {
    run(&["tests/inputs/g.csv"], "tests/expected/path_g.txt")
}
