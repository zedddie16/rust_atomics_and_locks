use std::rc::Rc;

fn main() {
    let a1 = [1, 2, 3];
    let b1 = a1.clone();
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr()); // same
    assert_ne!(a1.as_ptr(), b1.as_ptr()); // not
}
