use std::num::NonZeroU64;

use serde::Deserialize;

use crate::models::ship::requirements::Requirements;


#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EngineSymbol {
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveII,
    EngineHyperDriveI,
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


