pub mod shipyard_ship;
pub mod shipyard_transaction;

use serde::Deserialize;
use crate::utils::wrapper::TypeWrapper;
use super::ship::ship_type::ShipType;
use self::{shipyard_ship::ShipyardShip, shipyard_transaction::ShipyardTransaction};


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shipyard {
    /// The symbol of the shipyard. The symbol is the same as the waypoint where the shipyard is located.
    pub symbol: String,
    /// The list of ship types available for purchase at this shipyard.
    pub ship_types: Vec<TypeWrapper<ShipType>>,
    /// The list of recent transactions at this shipyard.
    pub transactions: Option<Vec<ShipyardTransaction>>,
    /// The ships that are currently available for purchase at the shipyard.
    pub ships: Option<Vec<ShipyardShip>>,
    /// The fee to modify a ship at this shipyard. This includes installing or removing modules and mounts on a ship. In the case of mounts, the fee is a flat rate per mount. In the case of modules, the fee is per slot the module occupies.
    pub modifications_fee: i64,
}