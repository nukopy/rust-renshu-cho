pub mod args;

use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(args: args::Args) -> MyResult<()> {
    println!("args: {:#?}", args);

    Ok(())
}
