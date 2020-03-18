#![feature(lint_reasons)]

#[expect(unused)]
//~^ ERROR: expect must have reason
fn main() {
    let x = 1;
}
