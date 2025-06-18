use spacetimedb::{table, Identity, ReducerContext, SpacetimeType, Table, TimeDuration, Timestamp, ScheduleAt};
use utils::{
    consts::{NOTIFICATION_PRUNE_AFTER_SECS, YRAL_SSR_TRUSTED_PRINCIPAL},
    identity_from_principal, validate_sender_identity, Error, Result,
};

#[table(name = notifications)]
pub struct Notification {
    #[index(btree)]
    user: Identity,
    #[primary_key]
    #[auto_inc]
    notification_id: u64,
    payload: NotificationType,
    created_at: Timestamp,
}

#[derive(serde::Deserialize, serde::Serialize, SpacetimeType)]
pub struct LikedPayload {
    creator_canister_id: String, // cant do spacetimetype for principal so string it is
    by_user_principal: String, // cant do spacetimetype for principal so string it is
    post_id: u64,
}

#[derive(serde::Deserialize, serde::Serialize, SpacetimeType)]
pub struct VideoUploadPayload {
    creator_canister_id: String, // cant do spacetimetype for principal so string it is
    video_id: u64,
}

#[derive(serde::Deserialize, serde::Serialize, SpacetimeType)]
pub enum NotificationType {
    Liked(LikedPayload),
    VideoUpload(VideoUploadPayload),
}

#[table(name = notification_prune_schedule, public, scheduled(prune_notifications))]
pub struct NotificationPruneSchedule {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: ScheduleAt,
}

#[spacetimedb::reducer]
pub fn add_notification(
    ctx: &ReducerContext,
    principal: String,
    payload: NotificationType,
) -> Result<()> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let principal = principal.parse().map_err(Error::from)?;
    let id = identity_from_principal(principal);

    let now = ctx.timestamp;
    ctx.db.notifications().insert(Notification {
        user: id,
        payload,
        created_at: now,
        notification_id: 0,
    });

    Ok(())
}

#[spacetimedb::reducer]
pub fn prune_notifications(
    ctx: &ReducerContext,
    _schedule: NotificationPruneSchedule,
) -> Result<()> {
    let cut_off = ctx.timestamp - TimeDuration::from_duration(NOTIFICATION_PRUNE_AFTER_SECS);

    for notification in ctx.db.notifications().iter() {
        if notification.created_at < cut_off {
            ctx.db.notifications().delete(notification);
        }
    }

    Ok(())
}

#[spacetimedb::reducer(init)]
pub fn init_notification_prune(ctx: &ReducerContext) {
    // Schedule the prune_notifications reducer to run in a loop every NOTIFICATION_PRUNE_AFTER_SECS.
    let loop_duration = TimeDuration::from_duration(NOTIFICATION_PRUNE_AFTER_SECS);

    // Insert the schedule row only if it hasn't been inserted yet (idempotent on replays).
    if ctx.db.notification_prune_schedule().count() == 0 {
        ctx.db.notification_prune_schedule().insert(NotificationPruneSchedule {
            scheduled_id: 0,
            scheduled_at: loop_duration.into(),
        });
    }
} 