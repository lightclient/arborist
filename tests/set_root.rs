mod common;
use arborist::Tree;

#[test]
fn basic() {
    let mut a = Tree::from_map(map! { 1 => 1, 2 => 2, 3 => 3 });
    a.set_root(5);

    let b = Tree::from_map(map! { 5 => 1, 10 => 2, 11 => 3 });
    assert_eq!(a, b);
}
