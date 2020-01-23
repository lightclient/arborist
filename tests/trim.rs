mod common;
use arborist::Tree;

#[test]
fn basic() {
    let a: Tree = map!(1, 2, 3, 4, 5).into();
    let b = map!(4, 5, 3).into();
    assert_eq!(a.trim(), b);
}
