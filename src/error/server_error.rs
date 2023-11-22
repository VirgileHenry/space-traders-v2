use std::fmt::Display;

use serde::Deserialize;
use super::code::ErrorCode;


#[derive(Deserialize, Debug, Clone)]
pub struct SpaceTraderError {
    message: String,
    code: ErrorCode,
}

impl Display for SpaceTraderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpaceTraders Error:\n")?;
        write!(f, "\tMessage: {}\n", self.message)?;
        write!(f, "\tError Code: {}\n", self.code)?;
        Ok(())
    }
}