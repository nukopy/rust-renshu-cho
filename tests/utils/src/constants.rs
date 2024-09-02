pub mod wc {
    pub const BINARY_NAME: &str = "wc";
    pub const EMPTY: &str = "tests/inputs/empty.txt";
    pub const FOX: &str = "tests/inputs/fox.txt";
    pub const ATLAMAL: &str = "tests/inputs/atlamal.txt";
}

pub mod uniq {
    use crate::types::uniq::Test;

    pub const BINARY_NAME: &str = "uniq";
    pub const EMPTY: Test = Test {
        input: "tests/inputs/empty.txt",
        out: "tests/expected/empty.txt.out",
        out_count: "tests/expected/empty.txt.c.out",
    };
    pub const ONE: Test = Test {
        input: "tests/inputs/one.txt",
        out: "tests/expected/one.txt.out",
        out_count: "tests/expected/one.txt.c.out",
    };
    pub const TWO: Test = Test {
        input: "tests/inputs/two.txt",
        out: "tests/expected/two.txt.out",
        out_count: "tests/expected/two.txt.c.out",
    };
    pub const THREE: Test = Test {
        input: "tests/inputs/three.txt",
        out: "tests/expected/three.txt.out",
        out_count: "tests/expected/three.txt.c.out",
    };
    pub const SKIP: Test = Test {
        input: "tests/inputs/skip.txt",
        out: "tests/expected/skip.txt.out",
        out_count: "tests/expected/skip.txt.c.out",
    };

    pub const T1: Test = Test {
        input: "tests/inputs/t1.txt",
        out: "tests/expected/t1.txt.out",
        out_count: "tests/expected/t1.txt.c.out",
    };

    pub const T2: Test = Test {
        input: "tests/inputs/t2.txt",
        out: "tests/expected/t2.txt.out",
        out_count: "tests/expected/t2.txt.c.out",
    };

    pub const T3: Test = Test {
        input: "tests/inputs/t3.txt",
        out: "tests/expected/t3.txt.out",
        out_count: "tests/expected/t3.txt.c.out",
    };

    pub const T4: Test = Test {
        input: "tests/inputs/t4.txt",
        out: "tests/expected/t4.txt.out",
        out_count: "tests/expected/t4.txt.c.out",
    };

    pub const T5: Test = Test {
        input: "tests/inputs/t5.txt",
        out: "tests/expected/t5.txt.out",
        out_count: "tests/expected/t5.txt.c.out",
    };

    pub const T6: Test = Test {
        input: "tests/inputs/t6.txt",
        out: "tests/expected/t6.txt.out",
        out_count: "tests/expected/t6.txt.c.out",
    };
}
