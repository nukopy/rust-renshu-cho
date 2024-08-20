mod args;

use clap::Parser;

use args::Args;

fn main() {
    let args = Args::parse();

    // extract args
    let text = args.text;
    let omit_newline = args.omit_newline;

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
