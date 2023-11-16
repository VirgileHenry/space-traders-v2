use serde::Deserialize;
use crate::schemas::faction::faction_symbol::FactionSymbol;

/// th faction that controls the waypoint.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WaypointFaction {
    /// The symbol of the faction.
    pub symbol: FactionSymbol
}