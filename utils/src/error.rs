use candid::types::principal::PrincipalError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid identity")]
    InvalidIdentity,
    #[error("invalid principal: {0}")]
    InvalidPrincipal(#[from] PrincipalError),
}

#[derive(Error, Debug)]
pub enum AddToWithdrawError {
    #[error(transparent)]
    Common(#[from] Error),
    #[error("withdrawal amount exceeds maximum allowed")]
    WithdrawalLimitExceeded,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
