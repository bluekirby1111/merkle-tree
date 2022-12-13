use std::{env, fs};

use consts::{MAX_CHUNK_SIZE, MIN_CHUNK_SIZE};
use error::Error;
use hash::hash_all_sha256;
use types::Node;

use crate::hash::sha256;

pub mod consts;
pub mod error;
pub mod hash;
pub mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    match fs::read(path.clone()) {
        Ok(data) => {
            let leaves: Vec<Node> = generate_leaves(data).unwrap();
            let root = generate_data_root(leaves).unwrap();
            println!("File merkle tree root: {:?}", root.hash);
        }
        Err(err) => println!("Error reading {} file: {}", path.clone(), err.to_string()),
    }
}

pub fn generate_leaves(data: Vec<u8>) -> Result<Vec<Node>, Error> {
    let mut data_chunks: Vec<&[u8]> = data.chunks(MAX_CHUNK_SIZE).collect();

    #[allow(unused_assignments)]
    let mut last_two = Vec::new();

    if data_chunks.len() > 1 && data_chunks.last().unwrap().len() < MIN_CHUNK_SIZE {
        last_two = data_chunks.split_off(data_chunks.len() - 2).concat();
        let chunk_size = last_two.len() / 2 + (last_two.len() % 2 != 0) as usize;
        data_chunks.append(&mut last_two.chunks(chunk_size).collect::<Vec<&[u8]>>());
    }

    let mut leaves = Vec::<Node>::new();
    for chunk in data_chunks.into_iter() {
        leaves.push(Node {
            hash: sha256(chunk),
            left: None,
            right: None,
        })
    }

    Ok(leaves)
}

pub fn build_layer(nodes: Vec<Node>) -> Result<Vec<Node>, Error> {
    let mut layer = Vec::<Node>::with_capacity(nodes.len() / 2 + (nodes.len() % 2 != 0) as usize);
    let mut nodes_iter = nodes.into_iter();
    while let Some(left) = nodes_iter.next() {
        if let Some(right) = nodes_iter.next() {
            layer.push(hash_branch(left, right).unwrap());
        } else {
            layer.push(left);
        }
    }
    Ok(layer)
}

pub fn hash_branch(left: Node, right: Node) -> Result<Node, Error> {
    let hash = hash_all_sha256(vec![&left.hash, &right.hash]);
    Ok(Node {
        hash,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    })
}

pub fn generate_data_root(mut nodes: Vec<Node>) -> Result<Node, Error> {
    while nodes.len() > 1 {
        nodes = build_layer(nodes).unwrap();
    }
    let root = nodes.pop().unwrap();
    Ok(root)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{build_layer, error::Error, generate_data_root, generate_leaves, types::Node};

    const ONE_MB_BINARY: &str = "res/1mb.bin";

    #[test]
    fn should_generate_leaves_correctly() -> Result<(), Error> {
        let data = fs::read(ONE_MB_BINARY).unwrap();
        let leaves = generate_leaves(data).unwrap();

        assert_eq!(leaves.len(), 4);
        assert_eq!(
            leaves[1],
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
        let leaves: Vec<Node> = generate_leaves(data).unwrap();
        let layer = build_layer(leaves).unwrap();
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
        let leaves: Vec<Node> = generate_leaves(data).unwrap();
        let root = generate_data_root(leaves).unwrap();
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
        // root id as calculate by arweave-js
        let leaves: Vec<Node> = generate_leaves(data).unwrap();
        let root = generate_data_root(leaves).unwrap();
        assert_eq!(
            root.hash,
            [
                145, 70, 225, 176, 161, 147, 24, 22, 11, 251, 142, 150, 71, 235, 137, 126, 212, 15,
                12, 235, 176, 115, 164, 63, 203, 170, 102, 24, 219, 217, 174, 74
            ]
        );
        Ok(())
    }
}
