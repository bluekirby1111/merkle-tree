use std::{env, fs};

use consts::{HASH_SIZE, MAX_CHUNK_SIZE, MIN_CHUNK_SIZE};
use error::Error;
use types::{Node, Tree};

use crate::{hash::sha256, types::MutableTree};

pub mod consts;
pub mod error;
pub mod hash;
pub mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    match fs::read(path.clone()) {
        Ok(data) => {
            let tree = generate_leaves(data, Tree::new(sha256)).unwrap();
            let root = calculate_root(tree).unwrap();
            println!("File merkle tree root: {:?}", root.hash);
        }
        Err(err) => println!("Error reading {} file: {}", path.clone(), err.to_string()),
    }
}

pub fn generate_leaves(data: Vec<u8>, mut tree: Tree) -> Result<Tree, Error> {
    let mut data_chunks: Vec<&[u8]> = data.chunks(MAX_CHUNK_SIZE).collect();

    #[allow(unused_assignments)]
    let mut last_two = Vec::new();

    let Some(last) = data_chunks.last() else {
        return Err(Error::NotFound);
    };

    if data_chunks.len() > 1 && last.len() < MIN_CHUNK_SIZE {
        last_two = data_chunks.split_off(data_chunks.len() - 2).concat();
        let chunk_size = last_two.len() / 2 + (last_two.len() % 2 != 0) as usize;
        data_chunks.append(&mut last_two.chunks(chunk_size).collect::<Vec<&[u8]>>());
    }

    for chunk in data_chunks.into_iter() {
        tree.insert_leaf(Node {
            hash: sha256(chunk),
            left: None,
            right: None,
        })
    }

    Ok(tree)
}

pub fn build_layer(tree: &Tree, nodes: Vec<Node>) -> Result<Vec<Node>, Error> {
    let mut layer = Vec::<Node>::with_capacity(nodes.len() / 2 + (nodes.len() % 2 != 0) as usize);
    let mut nodes_iter = nodes.into_iter();
    while let Some(left) = nodes_iter.next() {
        if let Some(right) = nodes_iter.next() {
            let Ok(node) = hash_branch(tree, left, right) else {
                return Err(Error::CouldNotHash)
            };
            layer.push(node);
        } else {
            layer.push(left);
        }
    }
    Ok(layer)
}

pub fn hash_branch(tree: &Tree, left: Node, right: Node) -> Result<Node, Error> {
    let left_hash = tree.hash(&left.hash);
    let right_hash = tree.hash(&right.hash);

    let hash = tree.hash(&[left_hash, right_hash].concat());
    Ok(Node {
        hash,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    })
}

pub fn calculate_root(tree: Tree) -> Result<Node, Error> {
    let mut nodes = tree.leaves.clone();
    while nodes.len() > 1 {
        nodes = match build_layer(&tree, nodes) {
            Ok(nodes) => nodes,
            Err(err) => return Err(err),
        }
    }
    match nodes.pop() {
        Some(node) => Ok(node),
        None => Err(Error::CouldNotHash),
    }
}

pub fn find_opening(tree: Tree, index: usize) -> [[u8; HASH_SIZE]; 2] {
    let (left_leaves, right_leaves) = tree.leaves.split_at(index);

    let mut left_tree = Tree::new(sha256);
    left_tree.insert_leaves(left_leaves.to_vec());
    let mut right_tree = Tree::new(sha256);
    right_tree.insert_leaves(right_leaves.to_vec());

    let left_tree_root = calculate_root(left_tree).unwrap_or_default();
    let right_tree_root = calculate_root(right_tree).unwrap_or_default();

    [left_tree_root.hash, right_tree_root.hash]
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        build_layer, calculate_root,
        error::Error,
        find_opening, generate_leaves,
        hash::sha256,
        types::{MutableTree, Node, Tree},
    };

    const ONE_MB_BINARY: &str = "res/1mb.bin";

    #[test]
    fn should_generate_leaves_correctly() -> Result<(), Error> {
        let data = fs::read(ONE_MB_BINARY).unwrap();
        let tree = generate_leaves(data, Tree::new(sha256)).unwrap();

        assert_eq!(tree.leaves.len(), 4);
        assert_eq!(
            tree.leaves[1],
            Node {
                hash: [
                    138, 57, 210, 171, 211, 153, 154, 183, 60, 52, 219, 36, 118, 132, 156, 221,
                    243, 3, 206, 56, 155, 53, 130, 104, 80, 249, 167, 0, 88, 155, 74, 144
                ],
                left: None,
                right: None
            }
        );
        Ok(())
    }

    #[test]
    fn should_build_layer_correctly() -> Result<(), Error> {
        let data = fs::read(ONE_MB_BINARY).unwrap();
        let tree = generate_leaves(data, Tree::new(sha256)).unwrap();
        let layer = build_layer(&tree, tree.leaves.clone()).unwrap();
        assert_eq!(
            layer[0].hash,
            [
                145, 252, 102, 63, 105, 86, 215, 195, 99, 148, 156, 153, 108, 133, 84, 33, 183,
                193, 195, 63, 31, 184, 3, 202, 117, 86, 21, 85, 217, 148, 216, 103
            ]
        );
        Ok(())
    }

    #[test]
    fn should_generate_root() -> Result<(), Error> {
        let data = fs::read(ONE_MB_BINARY).unwrap();
        let tree = generate_leaves(data, Tree::new(sha256)).unwrap();
        let root = calculate_root(tree).unwrap();
        assert_eq!(
            root.hash,
            [
                70, 162, 187, 117, 47, 193, 189, 49, 59, 4, 229, 155, 6, 73, 97, 9, 221, 249, 57,
                16, 47, 250, 10, 117, 227, 14, 82, 240, 110, 214, 97, 116
            ]
        );
        Ok(())
    }

    #[test]
    fn test_valid_root_small_last_chunk() -> Result<(), Error> {
        let data = vec![0; 256 * 1024 + 1];
        let tree = generate_leaves(data, Tree::new(sha256)).unwrap();
        let root = calculate_root(tree).unwrap();
        assert_eq!(
            root.hash,
            [
                145, 70, 225, 176, 161, 147, 24, 22, 11, 251, 142, 150, 71, 235, 137, 126, 212, 15,
                12, 235, 176, 115, 164, 63, 203, 170, 102, 24, 219, 217, 174, 74
            ]
        );
        Ok(())
    }

    #[test]
    fn should_find_opening() -> Result<(), Error> {
        let data = fs::read(ONE_MB_BINARY).unwrap();
        let tree = generate_leaves(data, Tree::new(sha256)).unwrap();
        let opening = find_opening(tree, 1);
        assert_eq!(
            opening,
            [
                [
                    138, 57, 210, 171, 211, 153, 154, 183, 60, 52, 219, 36, 118, 132, 156, 221,
                    243, 3, 206, 56, 155, 53, 130, 104, 80, 249, 167, 0, 88, 155, 74, 144
                ],
                [
                    34, 175, 149, 60, 99, 192, 81, 183, 12, 141, 75, 122, 35, 190, 185, 11, 208,
                    48, 144, 2, 7, 161, 120, 45, 221, 17, 106, 102, 150, 84, 13, 27
                ]
            ]
        );
        Ok(())
    }
}
