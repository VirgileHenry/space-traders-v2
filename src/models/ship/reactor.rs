use std::num::NonZeroU64;

use serde::Deserialize;

use crate::models::ship::requirements::Requirements;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReactorSymbol {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reactor {
    symbol: ReactorSymbol,
    name: String,
    description: String,
    condition: Option<u64>,
    power_output: NonZeroU64,
    requirements: Requirements,
}


