use std::time::Duration;

use spacetimedb::{table, Identity, ReducerContext, Table, TimeDuration, Timestamp};
use utils::{
    consts::YRAL_SSR_TRUSTED_PRINCIPAL, identity_from_principal, validate_sender_identity,
    AddToWithdrawError, Error, Result,
};

#[table(name = withdrawal_info, public)]
pub struct WithdrawalInfo {
    #[primary_key]
    user: Identity,
    withdrawal_amt_24h: u128,
    last_reset_at: Timestamp,
}

/// Add to overall withdrawal amount for a user,
/// this amount is reset every 24 hours.
/// if the overall withdrawal amount exceeds the maximum allowed,
/// an error is returned.
#[spacetimedb::reducer]
pub fn add_to_withdraw_amount(
    ctx: &ReducerContext,
    max_amount: u128,
    principal: String,
    amount: u128,
) -> Result<(), AddToWithdrawError> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let principal = principal.parse().map_err(Error::from)?;
    let id = identity_from_principal(principal);

    let Some(mut info) = ctx.db.withdrawal_info().user().find(id) else {
        ctx.db.withdrawal_info().insert(WithdrawalInfo {
            user: id,
            withdrawal_amt_24h: amount,
            last_reset_at: ctx.timestamp,
        });

        return Ok(());
    };

    if ctx.timestamp - TimeDuration::from_duration(Duration::from_secs(60)) >= info.last_reset_at {
        info.withdrawal_amt_24h = amount;
        info.last_reset_at = ctx.timestamp;
    } else {
        let new_amt = info.withdrawal_amt_24h.saturating_add(amount);
        info.withdrawal_amt_24h = (new_amt <= max_amount)
            .then_some(new_amt)
            .ok_or(AddToWithdrawError::WithdrawalLimitExceeded)?;
    }
    ctx.db.withdrawal_info().user().update(info);

    Ok(())
}
