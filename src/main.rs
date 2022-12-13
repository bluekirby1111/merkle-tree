use consts::{MAX_CHUNK_SIZE, MIN_CHUNK_SIZE};
use error::Error;
use types::Node;

use crate::hash::sha256;

pub mod consts;
pub mod error;
pub mod hash;
pub mod types;

fn main() {
    println!("Merklize some data!");
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
            hash: Some(sha256(chunk)),
            left: None,
            right: None,
        })
    }

    Ok(leaves)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{error::Error, generate_leaves, types::Node};

    const ONE_MB_BINARY: &str = "res/1mb.bin";

    #[test]
    fn should_generate_leaves_correctly() -> Result<(), Error> {
        let data = fs::read(ONE_MB_BINARY).unwrap();
        let leaves = generate_leaves(data).unwrap();

        assert_eq!(leaves.len(), 4);
        assert_eq!(
            leaves[1],
            Node {
                hash: Some([
                    138, 57, 210, 171, 211, 153, 154, 183, 60, 52, 219, 36, 118, 132, 156, 221,
                    243, 3, 206, 56, 155, 53, 130, 104, 80, 249, 167, 0, 88, 155, 74, 144
                ]),
                left: None,
                right: None
            }
        );
        Ok(())
    }
}
