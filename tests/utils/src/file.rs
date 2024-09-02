use std::fs::{self};

use crate::random::random_string;

pub fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}
