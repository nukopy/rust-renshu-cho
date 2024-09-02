use std::io::{BufRead, Read};

use crate::flags::Flags;

type Count = u64;

#[derive(Debug, Clone)]
pub struct Counter {
    pub filename: String,
    pub flags: Flags,
    pub lines: Count,
    pub words: Count,
    pub bytes: Count,
    pub chars: Count,
}

impl Counter {
    pub fn new(filename: String, flags: Flags) -> Self {
        Counter {
            filename,
            flags,
            lines: 0,
            words: 0,
            bytes: 0,
            chars: 0,
        }
    }

    pub fn count<R: BufRead>(&mut self, r: R) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = String::new();
        let mut reader = r.take(usize::MAX as u64); // BufRead から指定したバイト数を read する reader を生成
        reader.read_to_string(&mut buf)?;

        if self.flags.lines {
            let lines = Self::count_lines(buf.clone());
            self.lines = lines;
        }
        if self.flags.words {
            let words = Self::count_words(buf.clone());
            self.words = words;
        }
        if self.flags.bytes {
            let bytes = Self::count_bytes(buf.clone());
            self.bytes = bytes;
        }
        if self.flags.chars {
            let chars = Self::count_chars(buf.clone());
            self.chars = chars;
        }

        Ok(())
    }

    fn count_lines(buf: String) -> Count {
        let count = buf.lines().count() as Count;
        count
    }

    fn count_words(buf: String) -> Count {
        let count = buf.split_whitespace().count() as Count;
        count
    }

    fn count_bytes(buf: String) -> Count {
        buf.len() as Count
    }

    fn count_chars(buf: String) -> Count {
        let count = buf.chars().count() as Count;
        count
    }
}

pub struct Counters {
    pub flags: Flags,
    pub counters: Vec<Counter>,
    pub total_lines: Count,
    pub total_words: Count,
    pub total_bytes: Count,
    pub total_chars: Count,
}

impl Counters {
    pub fn new(flags: Flags, counters: Vec<Counter>) -> Self {
        Counters {
            flags,
            counters,
            total_lines: 0,
            total_words: 0,
            total_bytes: 0,
            total_chars: 0,
        }
    }

    pub fn count_total(&mut self) {
        for counter in &self.counters {
            self.total_lines += counter.lines;
            self.total_words += counter.words;
            self.total_bytes += counter.bytes;
            self.total_chars += counter.chars;
        }
    }
}
