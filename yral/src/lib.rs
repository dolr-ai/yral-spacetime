use spacetimedb::{reducer, Identity, ReducerContext, SpacetimeType, Table};

use crate::types::users::{user, SpaceTimeUser};

mod types;

fn validate_identity(ctx: &ReducerContext, principal: &str) -> Result<(), String> {
    let valid = ctx.sender == Identity::from_claims("https://auth.yral.com", principal);
    if !valid {
        return Err(format!("Invalid identity: {}", principal));
    }

    Ok(())
}

#[reducer]
fn register_new_user(ctx: &ReducerContext, principal: String) -> Result<(), String> {
    validate_identity(ctx, &principal)?;
    let Some(mut user) = ctx.db.user().id().find(ctx.sender) else {
        let mut new_user = SpaceTimeUser::default();
        new_user.id = ctx.sender;
        new_user.principal = principal;
        ctx.db.user().insert(new_user);
        return Ok(());
    };

    user.principal = principal;

    Ok(())
}

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) -> Result<(), String> {
    if let Some(_user) = ctx.db.user().id().find(ctx.sender) {
    } else {
        let mut new_user = SpaceTimeUser::default();

        new_user.id = ctx.sender;

        ctx.db.user().insert(new_user);
    }
    Ok(())
}
