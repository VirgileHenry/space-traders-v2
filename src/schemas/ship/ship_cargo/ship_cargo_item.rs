use std::num::NonZeroU64;
use serde::Deserialize;

/// The type of cargo item and the number of units.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipCargoItem {
    /// The unique identifier of the cargo item type.
    pub symbol: String,
    /// The name of the cargo item type.
    pub name: String,
    /// The description of the cargo item type.
    pub description: String,
    /// The number of units of the cargo item.
    pub units: NonZeroU64,
}

impl PartialEq for ShipCargoItem {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}