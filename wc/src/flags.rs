use crate::args::Args;

pub struct Flags {
    pub lines: bool,
    pub words: bool,
    pub bytes: bool,
    pub chars: bool,
}

impl Flags {
    /// wc コマンドのフラグ（option）の競合を解消する
    ///
    /// フラグの個数で競合をどう解消するかが決定される：
    ///
    /// - `-c` と `-m` は同時に true にならないので、4 つのフラグが同時に true になることはない
    /// - 3 つのフラグが指定された場合はそのまま
    /// - 1 ~ 2 つのフラグが指定された場合、他のフラグを false にする
    pub fn resolve_flag_conflicts(args: &Args) -> Self {
        let mut flags = Flags {
            lines: args.lines,
            words: args.words,
            bytes: args.bytes,
            chars: args.chars,
        };

        let true_count = [flags.lines, flags.words, flags.bytes, flags.chars]
            .iter()
            .filter(|&&x| x)
            .count();

        match true_count {
            0 => {
                // デフォルトの動作: 全てのフラグを true に設定
                flags.lines = true;
                flags.words = true;
                flags.bytes = true;
            }
            1..=3 => {
                // 1 ~ 3 つのフラグが指定された場合、他のフラグを false にする
                // args のデフォルト値が全て false なので、特に処理をする必要はない
            }
            4 => {
                // 4 つ全てが true の場合
                // ここには基本到達しないはず
                panic!("args.bytes and args.chars shouldn't be simultaneously true")
            }
            _ => {
                // この状況は理論上ありえないが、念のため
                panic!("Unexpected number of true flags");
            }
        }

        flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_flags_set() {
        let args = Args {
            files: vec![],
            lines: false,
            words: false,
            bytes: false,
            chars: false,
        };
        let flags = Flags::resolve_flag_conflicts(&args);
        assert!(flags.lines && flags.words && flags.bytes && !flags.chars);
    }

    #[test]
    fn test_one_flag_set() {
        let args = Args {
            files: vec![],
            lines: true,
            words: false,
            bytes: false,
            chars: false,
        };
        let flags = Flags::resolve_flag_conflicts(&args);
        assert!(flags.lines && !flags.words && !flags.bytes && !flags.chars);
    }

    #[test]
    fn test_two_flags_set() {
        let args = Args {
            files: vec![],
            lines: true,
            words: true,
            bytes: false,
            chars: false,
        };
        let flags = Flags::resolve_flag_conflicts(&args);
        assert!(flags.lines && flags.words && !flags.bytes && !flags.chars);
    }

    #[test]
    fn test_three_flags_set() {
        let args = Args {
            files: vec![],
            lines: true,
            words: true,
            bytes: true,
            chars: false,
        };
        let flags = Flags::resolve_flag_conflicts(&args);
        assert!(flags.lines && flags.words && flags.bytes && !flags.chars);
    }

    #[test]
    #[should_panic(expected = "args.bytes and args.chars shouldn't be simultaneously true")]
    fn test_bytes_and_chars_conflict() {
        let args = Args {
            files: vec![],
            lines: true,
            words: true,
            bytes: true,
            chars: true,
        };
        Flags::resolve_flag_conflicts(&args);
    }
}
