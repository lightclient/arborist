pub mod hash;

use hash::{hash, zero_hash};

use bonsai::{
    children, expand, first_leaf, last_leaf, log2, relative_depth, subtree_index_to_general,
};
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::convert::From;

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

    pub fn to_subtree(mut self, root: K) -> Self {
        self.map = self
            .map
            .into_iter()
            .map(|(k, v)| (subtree_index_to_general(root, k), v))
            .collect();

        self
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.map.insert(key, val)
    }

    pub fn keys(&self) -> BTreeSet<K> {
        self.map.keys().cloned().collect()
    }

    pub fn fill_subtree(&mut self, root: K, depth: u32, default: &V) {
        let mut keys: BinaryHeap<u128> = self
            .keys()
            .intersection(&self._leaf_keys(root, depth))
            .cloned()
            .collect();

        while let Some(key) = keys.pop() {
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
                keys.push(parent);
            }
        }
    }

    pub fn trim(mut self) -> Self {
        self._trim();
        self
    }

    pub fn raw_insert_bytes(&mut self, rooted_at: K, bytes: Vec<u8>) {
        let len = bytes.len() as K;
        let padded_len = len
            .checked_next_power_of_two()
            .expect("compiled code to fit in tree");
        let depth = log2(padded_len / 32);
        let first: K = first_leaf(rooted_at, depth);

        for i in (0..len).step_by(32) {
            let begin = i as usize;
            let end = min(i + 32, len) as usize;

            let chunk_len = if i + 32 < len {
                32
            } else {
                if end % 32 != 0 {
                    end % 32
                } else {
                    32
                }
            };

            let mut buf = [0u8; 32];
            buf[0..chunk_len].copy_from_slice(&bytes[begin..end]);

            self.map.insert(first + (i / 32), buf);
        }
    }

    pub fn insert_bytes(&mut self, rooted_at: K, bytes: Vec<u8>) {
        let len = bytes.len() as K;
        let padded_len = len
            .checked_next_power_of_two()
            .expect("compiled code to fit in tree");
        let depth = log2(padded_len / 32);

        self.raw_insert_bytes(rooted_at, bytes);
        self.fill_subtree(rooted_at, depth as u32, &[0; 32]);
        self._trim();
    }

    pub fn insert_subtree(&mut self, rooted_at: K, tree: Tree) {
        for (k, v) in tree.to_subtree(rooted_at).map {
            self.insert(k, v);
        }
    }

    fn _leaf_keys(&self, root: K, depth: u32) -> BTreeSet<K> {
        (first_leaf(root, depth as u128)..=last_leaf(root, depth as u128)).collect()
    }

    fn _trim(&mut self) {
        for key in self.keys() {
            let (left, right) = children(key);

            if self.map.contains_key(&left) || self.map.contains_key(&right) {
                self.map.remove(&key);
            }
        }
    }
}

impl From<Tree> for BTreeMap<K, V> {
    fn from(tree: Tree) -> BTreeMap<K, V> {
        tree.map
    }
}

impl From<BTreeMap<K, V>> for Tree {
    fn from(map: BTreeMap<K, V>) -> Tree {
        Tree { map }
    }
}
