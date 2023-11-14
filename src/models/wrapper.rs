use std::fmt::Debug;
use serde::Deserialize;

//* Wrappers are generic structs over common json schemes.
//* They allow easier deserialization and avoid boilerplates.
//* Otherwise, we would have to implement lot of redondant structs.

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

/// Wrapper arround the array scheme: [...]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArrayWrapper<T: Debug + Clone> {
    array: Vec<T>
}

impl<T: Debug + Clone> ArrayWrapper<T> {
    pub fn inner(self) -> Vec<T> {
        self.array
    }
}
