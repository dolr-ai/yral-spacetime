use spacetimedb::{log, reducer, table, ReducerContext, Table};

#[table(name = deleted_canisters, public)]
pub struct DeletedCanisters {
    pub canister: String,
    pub user_id: String,
}

#[reducer]
pub fn add_deleted_canister(
    ctx: &ReducerContext,
    canister: String,
    user_id: String,
) -> utils::Result<()> {
    utils::validate_sender_identity(ctx, utils::consts::OFFCHAIN_AGENT_TRUSTED_PRINCIPAL)
        .inspect_err(|err| {
            log::error!("validation failed: {err:?}");
        })?;

    let res = ctx
        .db
        .deleted_canisters()
        .try_insert(DeletedCanisters { canister, user_id });

    match res {
        Ok(..) => Ok(()),
        _ => unreachable!("This should never happen"),
    }
}
