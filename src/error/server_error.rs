use std::fmt::Display;

use serde::Deserialize;
use super::code::ErrorCode;


#[derive(Deserialize, Debug, Clone)]
pub struct ServerError {
    message: String,
    code: ErrorCode,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpaceTraders Error:\n")?;
        write!(f, "\tMessage: {}\n", self.message)?;
        write!(f, "\tError Code: {}\n", self.code)?;
        Ok(())
    }
}