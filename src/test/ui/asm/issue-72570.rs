// only-x86_64
// compile-flags: -Zsave-analysis
// This is also a regression test for #72960 and it needs the above flag.

#![feature(asm)]

fn main() {
    unsafe {
        asm!("", in("invalid") "".len());
        //~^ ERROR: invalid register `invalid`: unknown register
    }
}
