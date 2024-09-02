pub mod args;
pub mod io;

use std::io::{BufRead, Write};

use args::Args;
use io::{open_read, open_write};

mod debug;
#[allow(unused_imports)]
use debug::print_args;

fn print_line(
    output: &mut impl Write,
    line: &str,
    count: i32,
    enable_count: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if count == 0 {
        // 初回読み込み時の比較は必ず失敗するが、この if ブロックで出力を弾くことができる
        return Ok(());
    }

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
    let mut line = String::new();
    let mut count = 0;

    loop {
        let line_bytes = input.read_line(&mut line)?;
        if line_bytes == 0 {
            // EOF のとき、prev の情報を出力して loop を抜ける
            print_line(&mut output, prev_line.clone().as_str(), count, enable_count)?;
            break;
        }

        // prev と現在の行を比較する
        // もし等しくない場合、prev_line の集計が終わるので、出力を行い、count をリセットする
        if prev_line.trim_end() != line.trim_end() {
            // 等しくないときは prev の情報を出力して count をリセットする
            print_line(&mut output, prev_line.clone().as_str(), count, enable_count)?;
            prev_line = line.clone();
            count = 0;
        }

        // prev と現在の行が等しい場合は単に count をインクリメント
        // 等しくなかった場合も、現在の行のカウントを 1 から始めたいのでここでインクリメントして 1 にして次のループへ
        count += 1;

        // buffer は毎 loop ごとにリセットする
        line.clear();
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
