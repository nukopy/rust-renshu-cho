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
    /// head target files
    #[clap(value_name = "FILE", help = "Files to read", default_value = "-")]
    pub files: Vec<String>,

    /// head -n option
    #[clap(
        short = 'n',
        long = "lines",
        conflicts_with = "bytes", // -n と -c は同時に指定できないようにする
        help = "Print the first K lines instead of the first 10",
        default_value = "10"
    )]
    pub lines: usize,

    /// head -c option
    #[clap(
        short = 'c',
        long = "bytes",
        help = "Print the first K bytes of each file"
    )]
    pub bytes: Option<usize>,
}
