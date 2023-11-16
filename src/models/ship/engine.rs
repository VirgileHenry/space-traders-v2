use std::num::NonZeroU64;

use serde::Deserialize;

use crate::models::{ship::requirements::Requirements, resource::Resource};


#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EngineSymbol {
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveII,
    EngineHyperDriveI,
}

pub struct NotAnEngineError;
impl TryFrom<Resource> for EngineSymbol {
    type Error = NotAnEngineError;
    fn try_from(value: Resource) -> Result<Self, Self::Error> {
        match value {
            Resource::EngineHyperDriveI => Ok(Self::EngineHyperDriveI),
            Resource::EngineIonDriveII => Ok(Self::EngineIonDriveII),
            Resource::EngineIonDriveI => Ok(Self::EngineIonDriveI),
            Resource::EngineImpulseDriveI => Ok(Self::EngineImpulseDriveI),
            _ => Err(NotAnEngineError),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Engine {
    symbol: EngineSymbol,
    name: String,
    description: String,
    condition: Option<u64>,
    speed: NonZeroU64,
    requirements: Requirements,
}


