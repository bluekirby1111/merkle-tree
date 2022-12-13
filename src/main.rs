use error::Error;
use types::Node;

pub mod consts;
pub mod error;
pub mod types;

fn main() {
    println!("Merklize some data!");
}

pub fn generate_leaves(data: Vec<u8>) -> Result<Vec<Node>, Error> {
    todo!()
}

mod tests {}
