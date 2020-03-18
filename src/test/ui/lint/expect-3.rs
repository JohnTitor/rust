// should be errored due to missing feature gate.

#[expect(unused)]
//~^ ERROR: `expect` lint level is experimental [E0658]
fn main() {
    let x = 1;
}
