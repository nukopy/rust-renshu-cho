use crate::counter::{Counter, Counters};

pub struct Printer {
    counters: Counters,
}

impl Printer {
    pub fn new(counters: Counters) -> Self {
        Printer { counters }
    }

    pub fn output(&self) {
        // construct output string & output
        for counter in &self.counters.counters {
            let s = &self.construct_output_string(counter.clone());
            println!("{}", s);
        }
    }

    fn construct_output_string(&self, counter: Counter) -> String {
        // 8 文字幅の 3 列で構成される
        // format: "       <lines>       <words>       <bytes or chars>"
        let lines = if counter.flags.lines {
            format!("{:>8}", counter.lines)
        } else {
            String::from("")
        };
        let words = if counter.flags.words {
            format!("{:>8}", counter.words)
        } else {
            String::from("")
        };
        let bytes = if counter.flags.bytes {
            format!("{:>8}", counter.bytes)
        } else {
            String::from("")
        };
        let chars = if counter.flags.chars {
            format!("{:>8}", counter.chars)
        } else {
            String::from("")
        };
        let bytes_or_chars = if counter.flags.bytes { bytes } else { chars };

        format!("{}{}{} {}", lines, words, bytes_or_chars, counter.filename)
    }
}
