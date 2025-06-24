use spacetimedb::{table, ReducerContext, Table, TimeDuration, Timestamp};
use utils::{consts::YRAL_SSR_TRUSTED_PRINCIPAL, validate_sender_identity, Error};

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
) -> Result<(), Error> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let Some(mut prev) = ctx
        .db
        .dolr_airdrop_info()
        .user_principal()
        .find(user_principal.clone())
    else {
        ctx.db.dolr_airdrop_info().insert(DolrAirdropInfo {
            user_principal,
            airdrop_count_within_duration: 1,
            last_airdrop_at: now,
        });

        return Ok(());
    };

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
