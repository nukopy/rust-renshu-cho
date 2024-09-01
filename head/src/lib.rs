pub mod args;
pub mod fs;

use std::io::BufRead;

use args::Args;
use fs::open;

#[allow(dead_code)]
fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

#[allow(dead_code)]
fn print_args(args: Args) {
    println!("{:#?}", args);
    println!("Type of args.files: {}", type_of(&args.files));
    println!("Type of args.lines: {}", type_of(&args.lines));
    println!("Type of args.bytes: {}", type_of(&args.bytes));
}

fn head_bytes(mut r: Box<dyn BufRead>, bytes: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0u8; bytes];
    let bytes_read = r.read(&mut buf)?;
    buf.truncate(bytes_read);

    // 無効な UTF8 文字列まで出力したいため、String::from_utf8_lossy を使用している
    let output = String::from_utf8_lossy(&buf);
    print!("{}", output);

    Ok(())
}

fn head_lines(mut r: Box<dyn BufRead>, lines: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    for i in 0..lines {
        // 元の改行文字（LF or CRLF）を保持しつつ各行を標準出力に出力する
        match r.read_line(&mut line) {
            Ok(0) => {
                // EOF
                break;
            }
            Ok(_) => {
                print!("{}", line);
            }
            Err(e) => {
                let msg = format!("Error occurred on read line: {}", e);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    msg,
                )));
            }
        }
        line.clear();

        // 指定行数に達したら break
        if i + 1 == lines {
            break;
        }
    }

    Ok(())
}

fn head_file(args: Args, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    match open(filename) {
        Ok(r) => {
            if let Some(bytes) = args.bytes {
                // head -c
                if let Err(e) = head_bytes(r, bytes) {
                    eprintln!("Error occurred in head_bytes: {}", e);
                }
            } else {
                // head -n

                if let Err(e) = head_lines(r, args.lines) {
                    eprintln!("Error occurred in head_lines: {}", e);
                }
            }
        }
        Err(e) => {
            let msg = format!("Error occurred on opening file {}: {}", filename, e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                msg,
            )));
        }
    }

    Ok(())
}

fn head_files(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    for (i, filename) in args.files.iter().enumerate() {
        // ファイルが複数指定された場合の header
        if args.files.len() > 1 {
            // 最初に出力されるファイル以外のファイルは header 前に改行を行い、出力を見やすくする
            println!("{}==> {} <==", if i > 0 { "\n" } else { "" }, filename);
        }

        head_file(args.clone(), filename)?;
    }

    Ok(())
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // print_args(args);
    head_files(args)
}
