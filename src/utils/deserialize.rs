use serde::Deserialize;
use serde_json::Value;

use crate::{error::server_error::ServerError, models::wrapper::ErrorWrapper};

/// Will attempt to deserialize to T value,
/// then if it fails will attempt to deserialize to server value,
/// then if it fails return a json error.
pub(crate) fn deserialize<'de, T: serde::Deserialize<'de>>(value: Value) -> Result<T, crate::Error> {
    match T::deserialize(value.clone()) {
        Ok(t) => Ok(t),
        Err(des_error) => {
            match ErrorWrapper::<ServerError>::deserialize(value) {
                Ok(server_error) => Err(crate::Error::ServerError(server_error.inner())),
                Err(_) => Err(crate::Error::JsonParsingError(des_error)),
            }
        }
    }
}