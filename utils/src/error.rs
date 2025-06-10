use candid::types::principal::PrincipalError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid identity")]
    InvalidIdentity,
    #[error("invalid principal: {0}")]
    InvalidPrincipal(#[from] PrincipalError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
