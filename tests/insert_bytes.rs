mod common;
use arborist::Tree;

#[test]
fn raw_basic() {
    let mut data: Vec<u8> = val!(1).to_vec();
    data.extend(&val!(2));
    data.extend(&val!(3));

    let mut a = Tree::new();
    a.raw_insert_bytes(1, data);

    let b = map! { 4 => 1, 5 => 2, 6 => 3 }.into();

    assert_eq!(a, b);
}

#[test]
fn raw_partial_chunk() {
    let mut data: Vec<u8> = val!(1).to_vec();
    data.extend(&val!(2));
    data.extend(&[3; 31]);

    let mut a = Tree::new();
    a.raw_insert_bytes(1, data);

    let mut partial = [3; 32];
    partial[31] = 0;

    let mut b = map! { 4 => 1, 5 => 2 };
    b.insert(6, partial);

    assert_eq!(a, b.into());
}

#[test]
fn basic() {
    let mut data: Vec<u8> = val!(1).to_vec();
    data.extend(&val!(2));
    data.extend(&[3; 31]);

    let mut a = Tree::new();
    a.insert_bytes(1, data);

    let mut partial = [3; 32];
    partial[31] = 0;

    let mut b = map! { 4 => 1, 5 => 2, 7 => 0 };
    b.insert(6, partial);

    assert_eq!(a, b.into());
}
