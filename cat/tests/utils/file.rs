#![allow(dead_code)]

use std::{fs, os::unix::fs::PermissionsExt};

use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};

pub const DIRNAME_NOT_EXIST: &str = "./tests/tmp_not_exist";
pub const DIRNAME_CHMOD_000: &str = "./tests/tmp_chmod_000";

pub fn gen_not_exist_file(dirname: &str) -> String {
    loop {
        // random なファイル名を生成
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let filename = format!("{}/{}", dirname, filename);

        // ファイルが存在しない場合はファイル名を返す
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

pub fn gen_chmod_000_file(dirname: &str) -> Result<String> {
    // create file with 0o000 permission
    let filename = format!("{}/{}", dirname, "tmp.txt");
    fs::write(&filename, "hello")?;
    fs::set_permissions(&filename, fs::Permissions::from_mode(0o000))?;

    Ok(filename.to_string())
}

pub fn create_tmp_dirs(dirname: &str) -> Result<()> {
    fs::create_dir_all(dirname)?;

    Ok(())
}

pub fn remove_tmp_dirs(dirname: &str) -> Result<()> {
    fs::remove_dir_all(dirname)?;

    Ok(())
}

pub fn setup(dirname: &str) -> Result<()> {
    create_tmp_dirs(dirname)?;

    Ok(())
}

pub fn cleanup(dirname: &str) -> Result<()> {
    remove_tmp_dirs(dirname)?;

    Ok(())
}
