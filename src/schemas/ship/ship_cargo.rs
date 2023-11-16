pub mod ship_cargo_item;

use serde::Deserialize;
use self::ship_cargo_item::ShipCargoItem;

/// Ship cargo details.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipCargo {
    /// The max number of items that can be stored in the cargo hold.
    pub capacity: u64,
    /// The number of items currently stored in the cargo hold.
    pub units: u64,
    /// The items currently in the cargo hold.
    pub inventory: Vec<ShipCargoItem>,
}