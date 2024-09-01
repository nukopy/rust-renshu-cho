use std::fs::File;
use std::num::ParseIntError;

/**
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
 */

// ------------------------------
// sample_open_file
// ------------------------------

fn open_file(path: &str) -> Result<File, String> {
    let closure = |e: std::io::Error| format!("ファイル '{}' を開けませんでした: {}", path, e);
    File::open(path).map_err(|e: std::io::Error| closure(e))
}

fn sample_open_file() {
    match open_file("non_existent.txt") {
        Ok(file) => println!("ファイルを開きました: {:?}", file),
        Err(e) => println!("エラー: {}", e),
    }
}

// ------------------------------
// sample_parse
// ------------------------------

fn parse_and_double(s: &str) -> Result<i32, String> {
    let closure = |e: ParseIntError| e.to_string();
    s.parse::<i32>()
        .map_err(|e: ParseIntError| closure(e))
        .map(|n| n * 2)
}

fn sample_parse() {
    match parse_and_double("10") {
        Ok(n) => println!("結果: {}", n),
        Err(e) => println!("エラー: {}", e),
    }

    match parse_and_double("abc") {
        Ok(n) => println!("結果: {}", n),
        Err(e) => println!("エラー: {}", e),
    }
}

// ------------------------------
// sample_read_and_parse
// ------------------------------

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

fn read_and_parse(path: &str) -> Result<i32, MyError> {
    let closure_io = |e: std::io::Error| MyError::Io(e);
    let closure_parse = |e: ParseIntError| MyError::Parse(e);
    std::fs::read_to_string(path)
        .map_err(|e: std::io::Error| closure_io(e))
        .and_then(|contents| {
            contents
                .trim()
                .parse()
                .map_err(|e: ParseIntError| closure_parse(e))
        })
}

fn sample_read_and_parse() {
    match read_and_parse("number.txt") {
        Ok(n) => println!("読み取った数値: {}", n),
        Err(e) => match e {
            MyError::Io(e) => {
                println!("MyError::Io エラー: {:?}", e)
            }
            MyError::Parse(e) => {
                println!("MyError::Parse エラー: {:?}", e)
            }
        },
    }
}

fn main() {
    sample_open_file();
    sample_parse();
    sample_read_and_parse();
}
