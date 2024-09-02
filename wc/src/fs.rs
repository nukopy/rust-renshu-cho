use std::{
    error::Error,
    fs::File,
    io::{stdin, BufRead, BufReader},
};

pub fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => {
            let file = File::open(filename)?;
            Ok(Box::new(std::io::BufReader::new(file)))
        }
    }
}
