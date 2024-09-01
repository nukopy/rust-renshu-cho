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
            let s = &self.construct_one_file_count_string(counter.clone());
            println!("{}", s);
        }

        // if multiple files, print total counts
        if self.counters.counters.len() > 1 {
            let s = self.construct_total_count_string();
            println!("{}", s);
        }
    }

    fn construct_one_file_count_string(&self, counter: Counter) -> String {
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
        let f = if counter.filename == '-'.to_string() {
            "".to_string()
        } else {
            format!(" {}", &counter.filename)
        };

        format!("{}{}{}{}", lines, words, bytes_or_chars, f)
    }

    fn construct_total_count_string(&self) -> String {
        let lines = if self.counters.flags.lines {
            format!("{:>8}", self.counters.total_lines)
        } else {
            String::from("")
        };
        let words = if self.counters.flags.words {
            format!("{:>8}", self.counters.total_words)
        } else {
            String::from("")
        };
        let bytes = if self.counters.flags.bytes {
            format!("{:>8}", self.counters.total_bytes)
        } else {
            String::from("")
        };
        let chars = if self.counters.flags.chars {
            format!("{:>8}", self.counters.total_chars)
        } else {
            String::from("")
        };
        let bytes_or_chars = if self.counters.flags.bytes {
            bytes
        } else {
            chars
        };

        format!("{}{}{} total", lines, words, bytes_or_chars)
    }
}
