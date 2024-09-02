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
