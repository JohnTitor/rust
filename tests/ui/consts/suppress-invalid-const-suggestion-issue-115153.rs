// Regression test for <https://github.com/rust-lang/rust/issues/115153>.
// Make sure we don't emit an invalid suggestion which leads to a type dependency cycle.

fn foo(size: usize) {
    let size = size;
    let _ = [0; size];
    //~^ ERROR attempt to use a non-constant value in a constant
}

fn main() {}
