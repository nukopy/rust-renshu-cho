pub mod args;

use args::Args;

mod debug;
#[allow(unused_imports)]
use debug::print_args;

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    print_args(args);

    Ok(())
}
