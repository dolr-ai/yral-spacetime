use std::time::Duration;

use spacetimedb::{table, Identity, ReducerContext, Table, TimeDuration, Timestamp};
use utils::{
    consts::{NOTIFICATION_PRUNE_AFTER_SECS, YRAL_SSR_TRUSTED_PRINCIPAL},
    identity_from_principal, validate_sender_identity, Error, Result,
};

#[table(name = notifications, public)]
pub struct Notification {
    #[index(btree)]
    user: Identity,
    #[primary_key]
    #[auto_inc]
    notification_id: u64,
    payload: String,
    created_at: Timestamp,
}

#[spacetimedb::reducer]
pub fn add_notification(
    ctx: &ReducerContext,
    principal: String,
    notification_id: u64,
    payload: String,
) -> Result<()> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let principal = principal.parse().map_err(Error::from)?;
    let id = identity_from_principal(principal);

    let now = ctx.timestamp;
    ctx.db.notifications().insert(Notification {
        user: id,
        notification_id,
        payload,
        created_at: now,
    });

    let cut_off = now
        - TimeDuration::from_duration(Duration::from_secs(NOTIFICATION_PRUNE_AFTER_SECS));

    for old in ctx.db.notifications().user().filter(id) {
        if old.created_at < cut_off {
            ctx.db.notifications().delete(old);
        }
    }

    Ok(())
} 