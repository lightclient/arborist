mod common;
use arborist::Tree;

#[test]
fn basic() {
    let mut a: Tree = map! { 1 => 1, 2 => 2, 3 => 3 }.into();
    let b: Tree = map! { 1 => 1, 2 => 2, 3 => 3 }.into();
    let c: Tree = map! { 1 => 1, 2 => 1, 3 => 3, 4 => 2, 5 => 3 }.into();

    a.insert_subtree(2, b);

    assert_eq!(a, c);
}
