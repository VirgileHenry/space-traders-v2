//* Wrappers are generic structs over common json schemes.
//* They allow easier deserialization and avoid boilerplates.
//* Otherwise, we would have to implement lot of redondant structs.


use std::fmt::Debug;
use serde::Deserialize;
use crate::schemas::meta::Meta;

/// Wrapper arround the data scheme: { "data": {...} }
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataWrapper<T: Debug + Clone> {
    data: T,
}

impl<T: Debug + Clone> DataWrapper<T> {
    pub fn inner(self) -> T {
        self.data
    }
}

/// Wrapper arround the data scheme: { "data": [...], "meta": {...} }
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaginationWrapper<T: Debug + Clone> {
    data: Vec<T>,
    meta: Meta,
}

impl<T: Debug + Clone> PaginationWrapper<T> {
    pub fn inner(self) -> (Vec<T>, Meta) {
        (self.data, self.meta)
    }
}


