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
    /// text to print
    #[clap(value_name = "TEXT", help = "Text to print", required = true)]
    pub text: Vec<String>,

    /// echo `-n` option
    #[clap(
        short = 'n',
        help = "If true, omit a newline character '\\n' at the end of the output",
        default_value = "false"
    )]
    pub omit_newline: bool,
}
