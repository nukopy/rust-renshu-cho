pub mod args;
pub mod fs;

use std::{error::Error, io::BufRead};

macro_rules! println_numbered {
    ($($arg:tt)*) => {
        println!("{:6}\t{}", $($arg)*);
    }
}

pub fn run(args: args::Args) -> Result<(), Box<dyn Error>> {
    for file in args.files {
        let reader = fs::open(&file);
        match reader {
            Ok(r) => {
                if args.number_lines {
                    // cat with -n option
                    for (i, line) in r.lines().enumerate() {
                        println_numbered!(i + 1, line?);
                    }
                    continue;
                } else if args.number_nonblank_lines {
                    // cat with -b option
                    let mut n = 0;
                    for line in r.lines() {
                        let l = line?.clone();
                        if l.is_empty() {
                            println!();
                        } else {
                            println_numbered!(n + 1, l);
                            n += 1;
                        }
                    }
                    continue;
                } else {
                    // cat without options
                    for line in r.lines() {
                        println!("{}", line?);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open {}: {}", file, e)
            }
        }
    }

    Ok(())
}
