// check-pass
//
// The corresponding incremental compilation test can be found in
// `tests/incremental/issue-114416-expect-unused-inside-impl-block.rs`

#![feature(lint_reasons)]
#![warn(unused)]

struct OneUnused;
struct TwoUnused;

impl OneUnused {
    #[expect(unused)]
    fn unused() {}
}

impl TwoUnused {
    #[expect(unused)]
    fn unused1(){}

    // This unused method has `#[expect(unused)]`, so the compiler should not emit a warning.
    // This ui test was added after a regression in the compiler where it did not recognize multiple
    // `#[expect(unused)]` annotations inside of impl blocks.
    // issue 114416
    #[expect(unused)]
    fn unused2(){}
}

fn main() {
    let _ = OneUnused;
    let _ = TwoUnused;
}
