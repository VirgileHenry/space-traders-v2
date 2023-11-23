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

/// Wrapper arround the data scheme: { "type": {...} }
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypeWrapper<T: Debug + Clone> {
    #[serde(rename = "type")]
    _type: T,
}

impl<T: Debug + Clone> TypeWrapper<T> {
    pub fn inner(self) -> T {
        self._type
    }
}

/// Wrapper arround the error scheme: { "error": {...} }
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorWrapper<T: Debug + Clone> {
    error: T,
}

impl<T: Debug + Clone> ErrorWrapper<T> {
    pub fn inner(self) -> T {
        self.error
    }
}


/// Wrapper arround the data array scheme: { "data": [...] }
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArrayWrapper<T: Debug + Clone> {
    data: Vec<T>,
}

impl<T: Debug + Clone> ArrayWrapper<T> {
    pub fn inner(self) -> Vec<T> {
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


