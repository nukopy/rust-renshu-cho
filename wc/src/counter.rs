use std::io::BufRead;

use crate::flags::Flags;

type Count = i32;

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
        if self.flags.lines {
            let cnt = self.count_lines(r)?;
            self.lines = cnt;
        }
        /*
        if self.flags.words {
            self.count_words(&r)?;
        }
        if self.flags.bytes {
            self.count_bytes(&r)?;
        }
        if self.flags.chars {
            self.count_chars(&r)?;
        }
        */

        Ok(())
    }

    fn count_lines<R: BufRead>(&mut self, r: R) -> Result<Count, Box<dyn std::error::Error>> {
        let mut count = 0;
        for result in r.lines() {
            result?; // エラーチェック
            count += 1;
        }

        Ok(count)
    }

    /*
    fn count_words(&mut self, r: &Box<dyn BufRead>) -> Result<i32, Box<dyn std::error::Error>> {
        Ok(())
    }

    fn count_bytes(&mut self, r: &Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {}

    fn count_chars(&mut self, r: &Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {}
    */
}

pub struct Counters {
    pub counters: Vec<Counter>,
    pub total_lines: Count,
    pub total_words: Count,
    pub total_bytes: Count,
    pub total_chars: Count,
}

impl Counters {
    pub fn new(counters: Vec<Counter>) -> Self {
        Counters {
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
