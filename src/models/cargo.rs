use std::num::NonZeroU64;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cargo {
    capacity: u64,
    units: u64,
    inventory: Vec<CargoItem>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CargoItem {
    symbol: String,
    name: String,
    description: String,
    units: NonZeroU64,
}