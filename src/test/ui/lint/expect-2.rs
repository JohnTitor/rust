// check-pass

#![feature(lint_reasons)]

#[expect(unused, reason = "should be allowed.")]
#[expect(unused_parens, reason = "should be warned.")]
fn main() {
    let x = 1;
}
