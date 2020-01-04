mod common;
use arborist::hash::{hash, zero_hash};
use arborist::Tree;
use pretty_assertions::assert_eq;

#[test]
fn basic() {
    let mut map = map!(14, 15);

    let mut a = Tree::from_map(map!(14, 15));
    a.fill_subtree(1, 3, &[0u8; 32]);

    map.insert(7, hash(map.get(&14).unwrap(), map.get(&15).unwrap()));
    map.insert(6, zero_hash(&[0u8; 32], 1));
    map.insert(2, zero_hash(&[0u8; 32], 2));
    map.insert(3, hash(map.get(&6).unwrap(), map.get(&7).unwrap()));
    map.insert(1, hash(map.get(&2).unwrap(), map.get(&3).unwrap()));

    assert_eq!(a, Tree::from_map(map));
}
