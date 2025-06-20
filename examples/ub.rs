#[allow(dead_code)]
fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        // never happens
    }
} // &mut - exlusive borrowing
use std::cell::Cell;

#[allow(dead_code)]
fn f1(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        // might happen
    }
} // Cell on the other hand, allows interior mutability by copying and pasting changing value
// but can be used in one thread only.
fn main() {
    let a = [123, 456, 789];
    let b = unsafe { a.get_unchecked(4) };
    println!("{b}");
}
