use crate::{consts::HASH_SIZE, hash::sha256};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node {
    pub hash: [u8; HASH_SIZE],
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            hash: sha256(&[0]),
            left: None,
            right: None,
        }
    }
}

pub struct Tree {
    pub leaves: Vec<Node>,
    hash_fn: fn(&[u8]) -> [u8; HASH_SIZE],
}

pub trait MutableTree {
    fn new(hash_fn: fn(&[u8]) -> [u8; HASH_SIZE]) -> Self;
    fn insert_leaf(&mut self, node: Node);
    fn insert_leaves(&mut self, node: Vec<Node>);
    fn get(&self, index: usize) -> Option<&Node>;
    fn hash(&self, message: &[u8]) -> [u8; HASH_SIZE];
}

impl MutableTree for Tree {
    fn insert_leaf(&mut self, node: Node) {
        self.leaves.push(node)
    }

    fn insert_leaves(&mut self, nodes: Vec<Node>) {
        self.leaves.extend_from_slice(&nodes)
    }

    fn get(&self, index: usize) -> Option<&Node> {
        self.leaves.get(index)
    }

    // Different definitions possible for hash functions
    fn hash(&self, message: &[u8]) -> [u8; HASH_SIZE] {
        (self.hash_fn)(message)
    }

    fn new(hash_fn: fn(&[u8]) -> [u8; HASH_SIZE]) -> Self {
        Self {
            hash_fn,
            leaves: vec![],
        }
    }
}
