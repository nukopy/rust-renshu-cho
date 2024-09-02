pub mod args;
pub mod fs;

use std::io::{BufRead, Write};

use args::Args;
use fs::{open_read, open_write};

mod debug;
#[allow(unused_imports)]
use debug::print_args;

fn print_line(
    output: &mut impl Write,
    line: &str,
    count: i32,
    enable_count: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let s = if enable_count {
        format!("{:>4} {}", count, line)
    } else {
        line.to_string()
    };
    output.write_all(s.as_bytes())?;
    output.flush()?;

    Ok(())
}

fn uniq(
    mut input: impl BufRead,
    mut output: impl Write,
    enable_count: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut prev_line = String::new();
    let mut next_line = String::new();

    // 最初の一行を読み込む
    let line_bytes = input.read_line(&mut prev_line)?;
    if line_bytes == 0 {
        // EOF: input が empty のとき
        return Ok(());
    }

    // 二行目以降を読み込む
    let mut count = 1; // 最初の一行目の分のカウント
    loop {
        let line_bytes = input.read_line(&mut next_line)?;
        if line_bytes == 0 {
            // EOF のとき、prev の情報を出力してloop を抜ける
            print_line(&mut output, prev_line.clone().as_str(), count, enable_count)?;
            break;
        }

        // ここで prev_line の振る舞いが決まる
        // 比較のときは行末の改行文字は含まない
        if prev_line.trim_end() == next_line.trim_end() {
            count += 1;
            // このとき、prev_line は更新しない。
            // 出力されるときにカウントの最初の要素が出力される。
        } else {
            // 等しくないときは prev の情報を出力して count をリセットする
            print_line(&mut output, prev_line.clone().as_str(), count, enable_count)?;
            prev_line = next_line.clone();
            count = 1;
        }

        next_line.clear();
    }

    Ok(())
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // prepare input & output
    let input = match open_read(&args.in_file) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: {}", &args.in_file, e);
            return Err(e);
        }
    };
    let output = open_write(args.out_file.as_deref())?;
    let enable_count = args.count;

    // run uniq
    uniq(input, output, enable_count)
}
