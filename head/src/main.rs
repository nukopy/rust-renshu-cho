use clap::Parser;

use head::{args, run};

fn main() {
    // parse command-line arguments
    let args = args::Args::parse();

    // run
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
