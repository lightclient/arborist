mod common;
use arborist::Tree;

#[test]
fn basic() {
    let mut data: Vec<u8> = val!(1).to_vec();
    data.extend(&val!(2));
    data.extend(&val!(3));

    let mut a = Tree::new();
    a.insert_bytes(1, data);

    let b = Tree::from_map(map! { 4 => 1, 5 => 2, 6 => 3 });

    assert_eq!(a, b);
}

#[test]
fn partial_chunk() {
    let mut data: Vec<u8> = val!(1).to_vec();
    data.extend(&val!(2));
    data.extend(&[3; 31]);

    let mut a = Tree::new();
    a.insert_bytes(1, data);

    let mut partial = [3; 32];
    partial[31] = 0;

    let mut b = map! { 4 => 1, 5 => 2 };
    b.insert(6, partial);

    let b = Tree::from_map(b);

    assert_eq!(a, b);
}
