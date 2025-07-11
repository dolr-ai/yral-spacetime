// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit ).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct MarkAsReadArgs {
    pub principal: String,
    pub notification_id: u64,
}

impl From<MarkAsReadArgs> for super::Reducer {
    fn from(args: MarkAsReadArgs) -> Self {
        Self::MarkAsRead {
            principal: args.principal,
            notification_id: args.notification_id,
        }
    }
}

impl __sdk::InModule for MarkAsReadArgs {
    type Module = super::RemoteModule;
}

pub struct MarkAsReadCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `mark_as_read`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait mark_as_read {
    /// Request that the remote module invoke the reducer `mark_as_read` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_mark_as_read`] callbacks.
    fn mark_as_read(&self, principal: String, notification_id: u64) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `mark_as_read`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`MarkAsReadCallbackId`] can be passed to [`Self::remove_on_mark_as_read`]
    /// to cancel the callback.
    fn on_mark_as_read(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &String, &u64) + Send + 'static,
    ) -> MarkAsReadCallbackId;
    /// Cancel a callback previously registered by [`Self::on_mark_as_read`],
    /// causing it not to run in the future.
    fn remove_on_mark_as_read(&self, callback: MarkAsReadCallbackId);
}

impl mark_as_read for super::RemoteReducers {
    fn mark_as_read(&self, principal: String, notification_id: u64) -> __sdk::Result<()> {
        self.imp.call_reducer(
            "mark_as_read",
            MarkAsReadArgs {
                principal,
                notification_id,
            },
        )
    }
    fn on_mark_as_read(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &String, &u64) + Send + 'static,
    ) -> MarkAsReadCallbackId {
        MarkAsReadCallbackId(self.imp.on_reducer(
            "mark_as_read",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer:
                                super::Reducer::MarkAsRead {
                                    principal,
                                    notification_id,
                                },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, principal, notification_id)
            }),
        ))
    }
    fn remove_on_mark_as_read(&self, callback: MarkAsReadCallbackId) {
        self.imp.remove_on_reducer("mark_as_read", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `mark_as_read`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_mark_as_read {
    /// Set the call-reducer flags for the reducer `mark_as_read` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn mark_as_read(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_mark_as_read for super::SetReducerFlags {
    fn mark_as_read(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("mark_as_read", flags);
    }
}
