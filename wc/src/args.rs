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
    /// wc target files
    #[clap(value_name = "FILE", help = "Files to read on wc", default_value = "-")]
    pub files: Vec<String>,

    /// wc -l
    #[clap(
        short = 'l',
        long = "lines",
        help = "Print the number of lines in each input file",
        default_value_t = true
    )]
    pub lines: bool,

    /// wc -w
    #[clap(
        short = 'w',
        long = "words",
        help = "Print the number of words in each input file",
        default_value_t = true
    )]
    pub words: bool,

    /// wc -c
    #[clap(
        short = 'c',
        long = "bytes",
        conflicts_with = "chars", // -c と -m は同時に指定できないようにする
        help = "Print the number of bytes in each input file",
        default_value_t = true
    )]
    pub bytes: bool,

    /// wc -m
    #[clap(
        short = 'm',
        long = "chars",
        help = "Print the number of characters in each input file",
        default_value_t = false
    )]
    pub chars: bool,
}
