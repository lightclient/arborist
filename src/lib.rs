pub mod error;
mod hash;

use hash::{hash, zero_hash};

use bonsai::{children, expand, first_leaf, last_leaf, relative_depth, subtree_index_to_general};
use std::collections::{BinaryHeap, HashMap, HashSet};

pub type K = u128;
pub type V = [u8; 32];

#[derive(Debug, Default, PartialEq)]
pub struct Tree {
    map: HashMap<K, V>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn from_map(map: HashMap<K, V>) -> Self {
        Self { map }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn keys(&self) -> HashSet<K> {
        self.map.keys().cloned().collect()
    }

    fn leaf_keys(&self, root: K, depth: u32) -> HashSet<K> {
        (first_leaf(root, depth as u128)..last_leaf(root, depth as u128)).collect()
    }

    pub fn fill_subtree(&mut self, root: K, depth: u32, default: &V) {
        let mut keys: BinaryHeap<u128> = self
            .keys()
            .intersection(&self.leaf_keys(root, depth))
            .cloned()
            .collect();

        while let Some(key) = keys.pop() {
            if key <= root {
                break;
            }

            let (left, right, parent) = expand(key);

            if !self.map.contains_key(&parent) {
                let left = match self.map.get(&left) {
                    Some(k) => *k,
                    None => zero_hash(
                        default,
                        relative_depth(left, first_leaf(root, depth as u128)),
                    ),
                };

                let right = match self.map.get(&right) {
                    Some(k) => *k,
                    None => zero_hash(
                        default,
                        relative_depth(right, first_leaf(root, depth as u128)),
                    ),
                };

                self.map.insert(parent, hash(&left, &right));
                keys.push(parent);
            }
        }
    }

    pub fn into_branch(mut self) -> Self {
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
