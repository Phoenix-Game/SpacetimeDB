// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InsertCallerPkIdentityArgs {
    pub data: i32,
}

impl Reducer for InsertCallerPkIdentityArgs {
    const REDUCER_NAME: &'static str = "insert_caller_pk_identity";
}

#[allow(unused)]
pub fn insert_caller_pk_identity(data: i32) {
    InsertCallerPkIdentityArgs { data }.invoke();
}

#[allow(unused)]
pub fn on_insert_caller_pk_identity(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertCallerPkIdentityArgs> {
    InsertCallerPkIdentityArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertCallerPkIdentityArgs { data } = __args;
        __callback(__identity, __addr, __status, data);
    })
}

#[allow(unused)]
pub fn once_on_insert_caller_pk_identity(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertCallerPkIdentityArgs> {
    InsertCallerPkIdentityArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertCallerPkIdentityArgs { data } = __args;
        __callback(__identity, __addr, __status, data);
    })
}

#[allow(unused)]
pub fn remove_on_insert_caller_pk_identity(id: ReducerCallbackId<InsertCallerPkIdentityArgs>) {
    InsertCallerPkIdentityArgs::remove_on_reducer(id);
}
