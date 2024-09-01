pub mod args;
pub mod counter;
pub mod flags;
pub mod fs;
pub mod printer;

use args::Args;
use counter::{Counter, Counters};
use flags::Flags;
use fs::open;
use printer::Printer;

mod debug;
#[allow(unused_imports)]
use debug::print_args;

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // flag の処理
    let flags = Flags::resolve_flag_conflicts(&args);

    // 各ファイルのカウント
    let mut counter_list = vec![];
    for filename in args.files.into_iter() {
        match open(&filename) {
            Ok(r) => {
                let mut counter = Counter::new(filename, flags);
                counter.count(r)?; // wc の内部処理: フラグに応じてカウント
                counter_list.push(counter);
            }
            Err(e) => {
                eprintln!("{}: {}", &filename, e);
            }
        }
    }

    // トータルのカウント
    let mut counters = Counters::new(counter_list);
    counters.count_total();

    // 出力
    let printer = Printer::new(counters);
    printer.output();

    Ok(())
}
