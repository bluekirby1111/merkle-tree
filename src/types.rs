use crate::consts::HASH_SIZE;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub hash: Option<[u8; HASH_SIZE]>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
