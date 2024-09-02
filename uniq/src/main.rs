use clap::Parser;

use uniq::{args::Args, run};

fn main() {
    // parse command-line arguments
    let args = Args::parse();

    // run
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
