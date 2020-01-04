pub mod error;
pub mod hash;

use hash::{hash, zero_hash};

use bonsai::{children, expand, first_leaf, last_leaf, relative_depth, subtree_index_to_general};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashSet};

pub type K = u128;
pub type V = [u8; 32];

#[derive(Debug, Default, PartialEq)]
pub struct Tree {
    map: BTreeMap<K, V>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn from_map(map: BTreeMap<K, V>) -> Self {
        Self { map }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn keys(&self) -> HashSet<K> {
        self.map.keys().cloned().collect()
    }

    fn leaf_keys(&self, root: K, depth: u32) -> HashSet<K> {
        (first_leaf(root, depth as u128)..=last_leaf(root, depth as u128)).collect()
    }

    pub fn fill_subtree(&mut self, root: K, depth: u32, default: &V) {
        let mut keys: BinaryHeap<Reverse<u128>> = self
            .keys()
            .intersection(&self.leaf_keys(root, depth))
            .cloned()
            // mapping to reverse turn this into a min-heap
            .map(Reverse)
            .collect();

        while let Some(Reverse(key)) = keys.pop() {
            if key <= root {
                break;
            }

            let (left, right, parent) = expand(key);

            if !self.map.contains_key(&parent) {
                let mut get_or_insert = |n: K| -> V {
                    *self.map.entry(n).or_insert(zero_hash(
                        default,
                        relative_depth(n, first_leaf(root, depth as u128)),
                    ))
                };

                let left = get_or_insert(left);
                let right = get_or_insert(right);

                self.map.insert(parent, hash(&left, &right));
                keys.push(Reverse(parent));
            }
        }
    }

    pub fn trim(mut self) -> Self {
        for key in self.keys() {
            let (left, right) = children(key);

            if self.map.contains_key(&left) || self.map.contains_key(&right) {
                self.map.remove(&key);
            }
        }

        self
    }

    pub fn set_root(&mut self, root: K) {
        let keys = self.keys();
        for k in keys {
            let value = self.map.remove(&k).unwrap();
            self.map.insert(subtree_index_to_general(root, k), value);
        }
    }
}
