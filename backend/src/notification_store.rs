use candid::Principal;
use spacetimedb::{
    table, Identity, ReducerContext, ScheduleAt, SpacetimeType, Table, TimeDuration, Timestamp,
};
use utils::{
    consts::{NOTIFICATION_PRUNE_AFTER_SECS, YRAL_SSR_TRUSTED_PRINCIPAL},
    identity_from_principal, validate_sender_identity, Error, Result,
};

#[table(name = notification)]
pub struct Notification {
    #[primary_key]
    user: Identity,
    notifications: Vec<NotificationData>,
}

#[derive(SpacetimeType)]
struct NotificationData {
    notification_id: u64,
    payload: NotificationType,
    read: bool,
    created_at: Timestamp,
}

#[derive(serde::Deserialize, serde::Serialize, SpacetimeType)]
pub struct LikedPayload {
    by_user_principal: String, // cant do spacetimetype for principal so string it is
    post_id: u64,
}

#[derive(serde::Deserialize, serde::Serialize, SpacetimeType)]
pub struct VideoUploadPayload {
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

    let notifications = ctx.db.notification().user().find(id);

    match notifications {
        Some(mut notifications) => {
            notifications.notifications.push(NotificationData {
                notification_id: notifications.notifications.len() as u64,
                payload,
                created_at: now,
                read: false,
            });
            ctx.db.notification().user().update(notifications);
        }
        None => {
            let default_notification = Notification {
                user: id,
                notifications: vec![NotificationData {
                    notification_id: 0,
                    payload,
                    created_at: now,
                    read: false,
                }],
            };
            ctx.db.notification().insert(default_notification);
        }
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn mark_as_read(ctx: &ReducerContext, principal: String, notification_id: u64) -> Result<()> {
    validate_sender_identity(ctx, YRAL_SSR_TRUSTED_PRINCIPAL)?;

    let id = identity_from_principal(Principal::from_text(principal).unwrap());

    let notification = ctx
        .db
        .notification()
        .user()
        .find(id)
        .and_then(|mut notification| {
            notification
                .notifications
                .iter_mut()
                .find(|n| n.notification_id == notification_id)
                .map(|n| n.read = true);
            Some(notification)
        })
        .ok_or(Error::NotificationNotFound(notification_id))?;

    ctx.db.notification().user().update(notification);

    Ok(())
}

#[spacetimedb::reducer]
pub fn prune_notifications(
    ctx: &ReducerContext,
    _schedule: NotificationPruneSchedule,
) -> Result<()> {
    let cut_off = ctx.timestamp - TimeDuration::from_duration(NOTIFICATION_PRUNE_AFTER_SECS);

    for mut notification in ctx.db.notification().iter() {
        notification
            .notifications
            .retain(|n| n.created_at > cut_off);
        ctx.db.notification().user().update(notification);
    }

    Ok(())
}

#[spacetimedb::reducer(init)]
pub fn init_notification_prune(ctx: &ReducerContext) {
    // Schedule the prune_notifications reducer to run in a loop every NOTIFICATION_PRUNE_AFTER_SECS.
    let loop_duration = TimeDuration::from_duration(NOTIFICATION_PRUNE_AFTER_SECS);

    // Insert the schedule row only if it hasn't been inserted yet (idempotent on replays).
    if ctx.db.notification_prune_schedule().count() == 0 {
        ctx.db
            .notification_prune_schedule()
            .insert(NotificationPruneSchedule {
                scheduled_id: 0,
                scheduled_at: loop_duration.into(),
            });
    }
}
