use sha2::Digest;

use crate::consts::HASH_SIZE;

pub fn sha256(message: &[u8]) -> [u8; HASH_SIZE] {
    let mut context = sha2::Sha256::new();
    context.update(message);
    let mut result: [u8; HASH_SIZE] = [0; HASH_SIZE];
    result.copy_from_slice(context.finalize().as_ref());
    result
}
