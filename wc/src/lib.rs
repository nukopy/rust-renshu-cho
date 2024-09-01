pub mod args;
pub mod flags;
pub mod fs;

use args::Args;
use flags::Flags;
use fs::open;

mod debug;
#[allow(unused_imports)]
use debug::print_args;

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // flag の処理
    let _flags = Flags::resolve_flag_conflicts(&args);

    // 各ファイルの処理
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
