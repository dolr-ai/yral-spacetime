// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit ).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::notification_prune_schedule_type::NotificationPruneSchedule;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct PruneNotificationsArgs {
    pub schedule: NotificationPruneSchedule,
}

impl From<PruneNotificationsArgs> for super::Reducer {
    fn from(args: PruneNotificationsArgs) -> Self {
        Self::PruneNotifications {
            schedule: args.schedule,
        }
    }
}

impl __sdk::InModule for PruneNotificationsArgs {
    type Module = super::RemoteModule;
}

pub struct PruneNotificationsCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `prune_notifications`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait prune_notifications {
    /// Request that the remote module invoke the reducer `prune_notifications` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_prune_notifications`] callbacks.
    fn prune_notifications(&self, schedule: NotificationPruneSchedule) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `prune_notifications`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`PruneNotificationsCallbackId`] can be passed to [`Self::remove_on_prune_notifications`]
    /// to cancel the callback.
    fn on_prune_notifications(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &NotificationPruneSchedule) + Send + 'static,
    ) -> PruneNotificationsCallbackId;
    /// Cancel a callback previously registered by [`Self::on_prune_notifications`],
    /// causing it not to run in the future.
    fn remove_on_prune_notifications(&self, callback: PruneNotificationsCallbackId);
}

impl prune_notifications for super::RemoteReducers {
    fn prune_notifications(&self, schedule: NotificationPruneSchedule) -> __sdk::Result<()> {
        self.imp
            .call_reducer("prune_notifications", PruneNotificationsArgs { schedule })
    }
    fn on_prune_notifications(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &NotificationPruneSchedule)
            + Send
            + 'static,
    ) -> PruneNotificationsCallbackId {
        PruneNotificationsCallbackId(self.imp.on_reducer(
            "prune_notifications",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::PruneNotifications { schedule },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, schedule)
            }),
        ))
    }
    fn remove_on_prune_notifications(&self, callback: PruneNotificationsCallbackId) {
        self.imp
            .remove_on_reducer("prune_notifications", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `prune_notifications`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_prune_notifications {
    /// Set the call-reducer flags for the reducer `prune_notifications` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn prune_notifications(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_prune_notifications for super::SetReducerFlags {
    fn prune_notifications(&self, flags: __ws::CallReducerFlags) {
        self.imp
            .set_call_reducer_flags("prune_notifications", flags);
    }
}
