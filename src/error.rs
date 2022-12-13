use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not hash data")]
    CouldNotHash,

    #[error("No last chunk present")]
    NotFound,

    #[error("Unknown Error")]
    UnknownError,
}
