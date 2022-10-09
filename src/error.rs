use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("error nfa status {0}")]
    NfaStatus(u8),
    #[error("error uid length, Got {0}")]
    ErrorUidLen(u8),
    #[error(transparent)]
    LibloadingError(#[from] libloading::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
