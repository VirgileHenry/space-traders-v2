use serde::Deserialize;
use super::ship_role::ShipRole;

/// The public registration information of the ship
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    /// The agent's registered name of the ship
    pub name: String,
    /// The symbol of the faction the ship is registered with
    /// TODO: is this restricted to faction symbols?
    pub faction_symbol: String,
    /// The registered role of the ship
    pub role: ShipRole,
}