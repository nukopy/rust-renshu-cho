use std::{
    error::Error,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

pub fn open_read(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => {
            let file = File::open(filename)?;
            Ok(Box::new(std::io::BufReader::new(file)))
        }
    }
}

pub fn open_write(filename: Option<&str>) -> Result<Box<dyn Write>, Box<dyn Error>> {
    if let Some(f) = filename {
        // OUT_FILE 引数が与えられたときは、ファイルに書き込む
        let file = File::create(f)?;
        Ok(Box::new(BufWriter::new(file)))
    } else {
        // OUT_FILE 引数が与えられなかったときは、stdout に書き込む
        Ok(Box::new(BufWriter::new(stdout())))
    }
}
