pub mod wc {
    use anyhow::Result;
    use assert_cmd::Command;
    use std::fs;

    use crate::constants::wc::BINARY_NAME;

    pub fn run(args: &[&str], expected_file: &str) -> Result<()> {
        let expected = fs::read_to_string(expected_file)?;
        let output = Command::cargo_bin(BINARY_NAME)?
            .args(args)
            .output()
            .expect("fail");
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
        assert_eq!(stdout, expected);

        Ok(())
    }
}

pub mod uniq {
    use std::fs;

    use anyhow::Result;
    use assert_cmd::Command;
    use pretty_assertions::assert_eq;
    use tempfile::NamedTempFile;

    use crate::{constants::uniq::BINARY_NAME, types::uniq::Test};

    pub fn run(test: &Test) -> Result<()> {
        let expected = fs::read_to_string(test.out)?;
        let output = Command::cargo_bin(BINARY_NAME)?
            .arg(test.input)
            .output()
            .expect("fail");
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
        assert_eq!(stdout, expected);
        Ok(())
    }

    pub fn run_count(test: &Test) -> Result<()> {
        let expected = fs::read_to_string(test.out_count)?;
        let output = Command::cargo_bin(BINARY_NAME)?
            .args([test.input, "-c"])
            .output()
            .expect("fail");
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
        assert_eq!(stdout, expected);
        Ok(())
    }

    pub fn run_stdin(test: &Test) -> Result<()> {
        let input = fs::read_to_string(test.input)?;
        let expected = fs::read_to_string(test.out)?;
        let output = Command::cargo_bin(BINARY_NAME)?
            .write_stdin(input)
            .output()
            .expect("fail");
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
        assert_eq!(stdout, expected);
        Ok(())
    }

    pub fn run_stdin_count(test: &Test) -> Result<()> {
        let input = fs::read_to_string(test.input)?;
        let expected = fs::read_to_string(test.out_count)?;
        let output = Command::cargo_bin(BINARY_NAME)?
            .arg("--count")
            .write_stdin(input)
            .output()
            .expect("fail");
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
        assert_eq!(stdout, expected);
        Ok(())
    }

    pub fn run_outfile(test: &Test) -> Result<()> {
        let expected = fs::read_to_string(test.out)?;
        let outfile = NamedTempFile::new()?;
        let outpath = &outfile.path().to_str().unwrap();

        Command::cargo_bin(BINARY_NAME)?
            .args([test.input, outpath])
            .assert()
            .success()
            .stdout("");
        let contents = fs::read_to_string(outpath)?;
        assert_eq!(&expected, &contents);

        Ok(())
    }

    pub fn run_outfile_count(test: &Test) -> Result<()> {
        let outfile = NamedTempFile::new()?;
        let outpath = &outfile.path().to_str().unwrap();

        Command::cargo_bin(BINARY_NAME)?
            .args([test.input, outpath, "--count"])
            .assert()
            .success()
            .stdout("");

        let expected = fs::read_to_string(test.out_count)?;
        let contents = fs::read_to_string(outpath)?;
        assert_eq!(&expected, &contents);

        Ok(())
    }

    pub fn run_stdin_outfile_count(test: &Test) -> Result<()> {
        let input = fs::read_to_string(test.input)?;
        let outfile = NamedTempFile::new()?;
        let outpath = &outfile.path().to_str().unwrap();

        Command::cargo_bin(BINARY_NAME)?
            .args(["-", outpath, "-c"])
            .write_stdin(input)
            .assert()
            .stdout("");

        let expected = fs::read_to_string(test.out_count)?;
        let contents = fs::read_to_string(outpath)?;
        assert_eq!(&expected, &contents);

        Ok(())
    }
}

pub mod find {
    use std::fs;

    use anyhow::Result;
    use assert_cmd::Command;

    use crate::{constants::find::BINARY_NAME, file::format_file_name};

    pub fn run(args: &[&str], expected_file: &str) -> Result<()> {
        let file = format_file_name(expected_file);
        let contents = fs::read_to_string(file.as_ref())?;
        let mut expected: Vec<&str> = contents.split('\n').filter(|s| !s.is_empty()).collect();
        expected.sort();

        let cmd = Command::cargo_bin(BINARY_NAME)?
            .args(args)
            .assert()
            .success();
        let out = cmd.get_output();
        let stdout = String::from_utf8(out.stdout.clone())?;
        let mut lines: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();
        lines.sort();

        assert_eq!(lines, expected);

        Ok(())
    }
}
