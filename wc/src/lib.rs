pub mod args;
mod debug;
pub mod fs;

use args::Args;
#[allow(unused_imports)]
use debug::print_args;
use fs::open;

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    for filename in args.files.into_iter() {
        match open(&filename) {
            Ok(_r) => {}
            Err(e) => {
                eprintln!("{}: {}", &filename, e);
            }
        }
    }

    // print_args(args);
    Ok(())
}
