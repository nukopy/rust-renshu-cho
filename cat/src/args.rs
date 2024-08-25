use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = false, // 指定された引数がない場合は default の値を使う
)]
pub struct Args {
    /// Files to concatenate and print
    #[clap(
        value_name = "FILE",
        help = "Files to concatename and print",
        required = false,
        default_value = "-"
    )]
    pub files: Vec<String>,

    /// cat -n option
    #[clap(
        short = 'n',
        long = "number",
        conflicts_with = "number_nonblank_lines", // -n と -b は同時に指定できないようにする
        help = "If true, number all output lines",
        default_value = "false"
    )]
    pub number_lines: bool,

    /// cat -b option
    #[clap(
        short = 'b',
        long = "number-nonblank",
        help = "If true, number non-blank output lines",
        default_value = "false"
    )]
    pub number_nonblank_lines: bool,
}
