use std::time::Duration;

use spacetimedb::{table, Identity, ReducerContext, Table, TimeDuration, Timestamp};
use utils::{
    consts::YRAL_SSR_TRUSTED_PRINCIPAL, identity_from_principal, validate_sender_identity,
    AddToWithdrawError, Error, Result,
};

#[table(name = consts_info, public)]
pub struct ConstsInfo {
    // Withdraw limits
    web_enable_withdraw: bool,
    web_min_withdraw_per_txn: u128,
    web_max_withdraw_per_txn: u128,
    web_max_withdraw_24h: u128,

    // Signup reward limits
    web_new_user_signup_reward: u128,

    // Airdrop control/limits
    web_airdrop_enabled: bool,
    web_airdrop_amount_24h: u128,

    // Referral contol/limits
    web_referral_reward_enabled: bool,
    web_referee_reward: u128,
    web_referer_reward: u128,

    created_at: Timestamp,
}

/// Add to overall withdrawal amount for a user,
/// this amount is reset every 24 hours.
/// if the overall withdrawal amount exceeds the maximum allowed,
/// an error is returned.
#[spacetimedb::reducer]
pub fn get_withdraw_limits(
    ctx: &ReducerContext,
) -> Result<(), AddToWithdrawError> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let principal = principal.parse().map_err(Error::from)?;

    let Some(mut info) = ctx.db.consts_info().find(id);

    Ok(())
}


#[spacetimedb::reducer]
pub fn update_withdraw_limits(
    ctx: &ReducerContext,
) -> Result<(), AddToWithdrawError> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let principal = principal.parse().map_err(Error::from)?;

    // Validate input & identity

    let Some(mut info) = ctx.db.consts_info().update(id);

    Ok(())
}
