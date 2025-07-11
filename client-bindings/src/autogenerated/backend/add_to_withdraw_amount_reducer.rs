// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit ).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct AddToWithdrawAmountArgs {
    pub max_amount: u128,
    pub principal: String,
    pub amount: u128,
}

impl From<AddToWithdrawAmountArgs> for super::Reducer {
    fn from(args: AddToWithdrawAmountArgs) -> Self {
        Self::AddToWithdrawAmount {
            max_amount: args.max_amount,
            principal: args.principal,
            amount: args.amount,
        }
    }
}

impl __sdk::InModule for AddToWithdrawAmountArgs {
    type Module = super::RemoteModule;
}

pub struct AddToWithdrawAmountCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `add_to_withdraw_amount`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait add_to_withdraw_amount {
    /// Request that the remote module invoke the reducer `add_to_withdraw_amount` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_add_to_withdraw_amount`] callbacks.
    fn add_to_withdraw_amount(
        &self,
        max_amount: u128,
        principal: String,
        amount: u128,
    ) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `add_to_withdraw_amount`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`AddToWithdrawAmountCallbackId`] can be passed to [`Self::remove_on_add_to_withdraw_amount`]
    /// to cancel the callback.
    fn on_add_to_withdraw_amount(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &u128, &String, &u128) + Send + 'static,
    ) -> AddToWithdrawAmountCallbackId;
    /// Cancel a callback previously registered by [`Self::on_add_to_withdraw_amount`],
    /// causing it not to run in the future.
    fn remove_on_add_to_withdraw_amount(&self, callback: AddToWithdrawAmountCallbackId);
}

impl add_to_withdraw_amount for super::RemoteReducers {
    fn add_to_withdraw_amount(
        &self,
        max_amount: u128,
        principal: String,
        amount: u128,
    ) -> __sdk::Result<()> {
        self.imp.call_reducer(
            "add_to_withdraw_amount",
            AddToWithdrawAmountArgs {
                max_amount,
                principal,
                amount,
            },
        )
    }
    fn on_add_to_withdraw_amount(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &u128, &String, &u128) + Send + 'static,
    ) -> AddToWithdrawAmountCallbackId {
        AddToWithdrawAmountCallbackId(self.imp.on_reducer(
            "add_to_withdraw_amount",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer:
                                super::Reducer::AddToWithdrawAmount {
                                    max_amount,
                                    principal,
                                    amount,
                                },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, max_amount, principal, amount)
            }),
        ))
    }
    fn remove_on_add_to_withdraw_amount(&self, callback: AddToWithdrawAmountCallbackId) {
        self.imp
            .remove_on_reducer("add_to_withdraw_amount", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `add_to_withdraw_amount`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_add_to_withdraw_amount {
    /// Set the call-reducer flags for the reducer `add_to_withdraw_amount` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn add_to_withdraw_amount(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_add_to_withdraw_amount for super::SetReducerFlags {
    fn add_to_withdraw_amount(&self, flags: __ws::CallReducerFlags) {
        self.imp
            .set_call_reducer_flags("add_to_withdraw_amount", flags);
    }
}
