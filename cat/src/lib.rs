pub mod args;
pub mod fs;

use std::error::Error;

pub fn run(args: args::Args) -> Result<(), Box<dyn Error>> {
    for file in args.files {
        let reader = fs::open(&file);
        match reader {
            Ok(_reader) => {
                println!("Opened: {}", file);
            }
            Err(e) => {
                eprintln!("Failed to open {}: {}", file, e)
            }
        }
    }

    Ok(())
}
