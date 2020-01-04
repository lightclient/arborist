mod common;
use arborist::Tree;

#[test]
fn basic() {
    let a = Tree::from_map(map!(1, 2, 3, 4, 5));
    let b = Tree::from_map(map!(4, 5, 3));
    assert_eq!(a.trim(), b);
}
