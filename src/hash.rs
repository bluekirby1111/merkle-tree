use sha2::Digest;

pub fn sha256(message: &[u8]) -> [u8; 32] {
    let mut context = sha2::Sha256::new();
    context.update(message);
    let mut result: [u8; 32] = [0; 32];
    result.copy_from_slice(context.finalize().as_ref());
    result
}
