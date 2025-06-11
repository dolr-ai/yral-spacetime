use spacetimedb::{Identity, ReducerContext};

pub mod consts;
mod error;

pub use error::*;

pub fn identity_from_principal(principal: &str) -> Identity {
    Identity::from_claims(consts::AUTH_JWT_ISS, principal)
}

pub fn validate_sender_identity(ctx: &ReducerContext, principal: &str) -> Result<()> {
    let valid = ctx.sender == identity_from_principal(principal);
    if !valid {
        return Err(Error::InvalidIdentity);
    }

    Ok(())
}
