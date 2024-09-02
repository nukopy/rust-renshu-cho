use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = false, // 指定された引数がない場合は default の値を使う
)]
pub struct Args {
    /// uniq <IN_FILE>
    #[clap(value_name = "IN_FILE", help = "Files to read", default_value = "-")]
    pub in_file: String,

    /// uniq <OUT_FILE>
    #[clap(value_name = "OUT_FILE", help = "Files to output")]
    pub out_file: Option<String>,

    /// uniq -c
    #[clap(
        short = 'c',
        long = "count",
        help = "Adds a number at the start of each output line. This number shows how many times that line appeared in the input. The number is followed by a space.",
        default_value_t = false
    )]
    pub count: bool,
}
