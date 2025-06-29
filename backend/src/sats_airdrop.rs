use spacetimedb::{table, ReducerContext, Table, Timestamp};
use utils::{consts::YRAL_SSR_TRUSTED_PRINCIPAL, validate_sender_identity, SatsAirdropError};

#[table(name = sats_airdrop_info, public)]
pub struct SatsAirdropInfo {
    #[primary_key]
    user_principal: String,
    last_airdrop_at: Timestamp,
}

#[spacetimedb::reducer]
pub fn mark_sats_airdrop_claimed(
    ctx: &ReducerContext,
    user_principal: String,
    now: Timestamp,
    // last airdrop timestamp as read by the caller
    last_airdrop_at: Option<Timestamp>,
) -> Result<(), SatsAirdropError> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let prev = ctx
        .db
        .sats_airdrop_info()
        .user_principal()
        .find(user_principal.clone());

    let (mut prev, last_airdrop_at) = match (prev, last_airdrop_at) {
        (Some(..), None) | (None, Some(..)) => {
            return Err(SatsAirdropError::LastAirdropTimeMismatch)
        }
        (None, None) => {
            ctx.db.sats_airdrop_info().insert(SatsAirdropInfo {
                user_principal,
                last_airdrop_at: now,
            });

            return Ok(());
        }
        (Some(prev), Some(last_airdrop_at)) => (prev, last_airdrop_at),
    };

    if last_airdrop_at != prev.last_airdrop_at {
        return Err(SatsAirdropError::LastAirdropTimeMismatch);
    }

    prev.last_airdrop_at = now;

    ctx.db.sats_airdrop_info().user_principal().update(prev);

    Ok(())
}
