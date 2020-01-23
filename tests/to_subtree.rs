mod common;
use arborist::Tree;

#[test]
fn basic() {
    let a: Tree = map! { 1 => 1, 2 => 2, 3 => 3 }.into();
    let b: Tree = map! { 5 => 1, 10 => 2, 11 => 3 }.into();
    assert_eq!(a.to_subtree(5), b);
}
