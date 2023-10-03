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
pub struct InsertVecIdentityArgs {
    pub i: Vec<Identity>,
}

impl Reducer for InsertVecIdentityArgs {
    const REDUCER_NAME: &'static str = "insert_vec_identity";
}

#[allow(unused)]
pub fn insert_vec_identity(i: Vec<Identity>) {
    InsertVecIdentityArgs { i }.invoke();
}

#[allow(unused)]
pub fn on_insert_vec_identity(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<Identity>) + Send + 'static,
) -> ReducerCallbackId<InsertVecIdentityArgs> {
    InsertVecIdentityArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecIdentityArgs { i } = __args;
        __callback(__identity, __addr, __status, i);
    })
}

#[allow(unused)]
pub fn once_on_insert_vec_identity(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<Identity>) + Send + 'static,
) -> ReducerCallbackId<InsertVecIdentityArgs> {
    InsertVecIdentityArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecIdentityArgs { i } = __args;
        __callback(__identity, __addr, __status, i);
    })
}

#[allow(unused)]
pub fn remove_on_insert_vec_identity(id: ReducerCallbackId<InsertVecIdentityArgs>) {
    InsertVecIdentityArgs::remove_on_reducer(id);
}