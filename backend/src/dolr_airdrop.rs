use std::{fmt::Display, time::Duration};

use spacetimedb::{table, Identity, ReducerContext, TimeDuration, Timestamp};

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
    duration: Duration,
    max_count_allowed: u64,
) -> Result<(), impl Display> {
    // TODO: add identity validation before merging
    let now = ctx.timestamp;

    let Some(mut prev) = ctx
        .db
        .dolr_airdrop_info()
        .user_principal()
        .find(user_principal)
    else {
        ctx.db.dolr_airdrop_info().insert(DolrAirdropInfo {
            user_principal,
            airdrop_count_within_duration: 1,
            last_airdrop_at: now,
        });

        return Ok(());
    };

    let next_airdrop_available_after = prev.last_airdrop_at + TimeDuration::from_duration(duration);

    if now < next_airdrop_available_after {
        if prev.airdrop_count_within_duration >= max_count_allowed {
            return Err("Max airdrop claim count reached");
        }
        prev.airdrop_count_within_duration += 1;
    } else {
        prev.airdrop_count_within_duration = 1;
        prev.last_airdrop_at = now;
    }

    ctx.db.dolr_airdrop_info().user_principal().update(prev);

    Ok(())
}
