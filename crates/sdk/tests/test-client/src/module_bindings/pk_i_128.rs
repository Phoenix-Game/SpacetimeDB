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
pub struct PkI128 {
    pub n: i128,
    pub data: i32,
}

impl TableType for PkI128 {
    const TABLE_NAME: &'static str = "PkI128";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for PkI128 {
    type PrimaryKey = i128;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.n
    }
}

impl PkI128 {
    #[allow(unused)]
    pub fn filter_by_n(n: i128) -> Option<Self> {
        Self::find(|row| row.n == n)
    }
    #[allow(unused)]
    pub fn filter_by_data(data: i32) -> TableIter<Self> {
        Self::filter(|row| row.data == data)
    }
}
