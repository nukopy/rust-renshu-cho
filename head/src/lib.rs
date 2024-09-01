pub mod args;
pub mod fs;

use std::error::Error;

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

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // print_args(args);
    for filename in args.files {
        match open(&filename) {
            Ok(b) => {
                println!("Opened {}", filename);
            }
            Err(e) => {
                eprintln!("Error occurred on opening file {}: {}", filename, e);
            }
        }
    }

    Ok(())
}
