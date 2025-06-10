use std::time::Duration;

use spacetimedb::{reducer, table, Identity, ReducerContext, ScheduleAt, Table, TimeDuration};
use utils::{
    consts::YRAL_SSR_TRUSTED_PRINCIPAL, identity_from_principal, validate_sender_identity, Result,
};

#[table(name = withdrawal_info, public)]
pub struct WithdrawalInfo {
    #[primary_key]
    user: Identity,
    withdrawal_amt_24h: u128,
}

#[table(name = reset_withdrawal_schedule, scheduled(reset_withdrawal_amt))]
pub struct ResetWithdrawalSchedule {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: ScheduleAt,
}

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    let withdrawal_reset_interval = TimeDuration::from_duration(Duration::from_secs(24 * 60 * 60)); // 24 hours
    ctx.db
        .reset_withdrawal_schedule()
        .insert(ResetWithdrawalSchedule {
            scheduled_id: 0,
            scheduled_at: withdrawal_reset_interval.into(),
        });
}

#[reducer]
fn reset_withdrawal_amt(ctx: &ReducerContext, _arg: ResetWithdrawalSchedule) {
    if ctx.sender != ctx.identity() {
        return;
    }

    log::info!("Resetting withdrawal amounts for all users!");

    ctx.db.withdrawal_info().iter().for_each(|info| {
        ctx.db.withdrawal_info().delete(info);
    })
}

/// Add to overall withdrawal amount for a user,
/// this amount is reset every 24 hours.
#[spacetimedb::reducer]
pub fn add_to_withdraw_amount(ctx: &ReducerContext, principal: String, amount: u128) -> Result<()> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let principal = principal.parse()?;
    let id = identity_from_principal(principal);

    if let Some(mut info) = ctx.db.withdrawal_info().user().find(id) {
        info.withdrawal_amt_24h = info.withdrawal_amt_24h.saturating_add(amount);
        ctx.db.withdrawal_info().user().update(info);
    } else {
        ctx.db.withdrawal_info().insert(WithdrawalInfo {
            user: id,
            withdrawal_amt_24h: amount,
        });
    }

    Ok(())
}
