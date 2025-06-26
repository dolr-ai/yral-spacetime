use spacetimedb::{table, ReducerContext, Table, TimeDuration, Timestamp};
use utils::{consts::YRAL_SSR_TRUSTED_PRINCIPAL, validate_sender_identity, DolrAirdropError};

#[table(name = dolr_airdrop_info, public)]
pub struct DolrAirdropInfo {
    #[primary_key]
    user_principal: String,
    airdrop_count_within_duration: u64,
    last_airdrop_at: Timestamp,
}

#[spacetimedb::reducer]
pub fn mark_airdrop_claimed(
    ctx: &ReducerContext,
    user_principal: String,
    duration: TimeDuration,
    // take from the caller to avoid issues with lag
    now: Timestamp,
    // last airdrop timestamp as read by the caller
    last_airdrop_at: Option<Timestamp>,
) -> Result<(), DolrAirdropError> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let prev = ctx
        .db
        .dolr_airdrop_info()
        .user_principal()
        .find(user_principal.clone());

    let (mut prev, last_airdrop_at) = match (prev, last_airdrop_at) {
        (Some(..), None) | (None, Some(..)) => {
            return Err(DolrAirdropError::LastAirdropTimeMismatch)
        }
        (None, None) => {
            ctx.db.dolr_airdrop_info().insert(DolrAirdropInfo {
                user_principal,
                airdrop_count_within_duration: 1,
                last_airdrop_at: now,
            });

            return Ok(());
        }
        (Some(prev), Some(last_airdrop_at)) => (prev, last_airdrop_at),
    };

    if last_airdrop_at != prev.last_airdrop_at {
        return Err(DolrAirdropError::LastAirdropTimeMismatch);
    }

    let next_airdrop_available_after = prev.last_airdrop_at + duration;

    if now < next_airdrop_available_after {
        prev.airdrop_count_within_duration += 1;
    } else {
        prev.airdrop_count_within_duration = 1;
        prev.last_airdrop_at = now;
    }

    ctx.db.dolr_airdrop_info().user_principal().update(prev);

    Ok(())
}
