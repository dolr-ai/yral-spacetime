// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit ).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct MarkAirdropClaimedArgs {
    pub user_principal: String,
    pub duration: __sdk::TimeDuration,
    pub now: __sdk::Timestamp,
}

impl From<MarkAirdropClaimedArgs> for super::Reducer {
    fn from(args: MarkAirdropClaimedArgs) -> Self {
        Self::MarkAirdropClaimed {
            user_principal: args.user_principal,
            duration: args.duration,
            now: args.now,
        }
    }
}

impl __sdk::InModule for MarkAirdropClaimedArgs {
    type Module = super::RemoteModule;
}

pub struct MarkAirdropClaimedCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `mark_airdrop_claimed`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait mark_airdrop_claimed {
    /// Request that the remote module invoke the reducer `mark_airdrop_claimed` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_mark_airdrop_claimed`] callbacks.
    fn mark_airdrop_claimed(
        &self,
        user_principal: String,
        duration: __sdk::TimeDuration,
        now: __sdk::Timestamp,
    ) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `mark_airdrop_claimed`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`MarkAirdropClaimedCallbackId`] can be passed to [`Self::remove_on_mark_airdrop_claimed`]
    /// to cancel the callback.
    fn on_mark_airdrop_claimed(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &String, &__sdk::TimeDuration, &__sdk::Timestamp)
            + Send
            + 'static,
    ) -> MarkAirdropClaimedCallbackId;
    /// Cancel a callback previously registered by [`Self::on_mark_airdrop_claimed`],
    /// causing it not to run in the future.
    fn remove_on_mark_airdrop_claimed(&self, callback: MarkAirdropClaimedCallbackId);
}

impl mark_airdrop_claimed for super::RemoteReducers {
    fn mark_airdrop_claimed(
        &self,
        user_principal: String,
        duration: __sdk::TimeDuration,
        now: __sdk::Timestamp,
    ) -> __sdk::Result<()> {
        self.imp.call_reducer(
            "mark_airdrop_claimed",
            MarkAirdropClaimedArgs {
                user_principal,
                duration,
                now,
            },
        )
    }
    fn on_mark_airdrop_claimed(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &String, &__sdk::TimeDuration, &__sdk::Timestamp)
            + Send
            + 'static,
    ) -> MarkAirdropClaimedCallbackId {
        MarkAirdropClaimedCallbackId(self.imp.on_reducer(
            "mark_airdrop_claimed",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer:
                                super::Reducer::MarkAirdropClaimed {
                                    user_principal,
                                    duration,
                                    now,
                                },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, user_principal, duration, now)
            }),
        ))
    }
    fn remove_on_mark_airdrop_claimed(&self, callback: MarkAirdropClaimedCallbackId) {
        self.imp
            .remove_on_reducer("mark_airdrop_claimed", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `mark_airdrop_claimed`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_mark_airdrop_claimed {
    /// Set the call-reducer flags for the reducer `mark_airdrop_claimed` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn mark_airdrop_claimed(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_mark_airdrop_claimed for super::SetReducerFlags {
    fn mark_airdrop_claimed(&self, flags: __ws::CallReducerFlags) {
        self.imp
            .set_call_reducer_flags("mark_airdrop_claimed", flags);
    }
}
