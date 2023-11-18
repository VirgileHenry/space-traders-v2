use serde::Deserialize;

use crate::schemas::system::system_type::SystemType;

/// Details of a system was that scanned.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct System {
    /// The symbol of the system.
    pub symbol: String,
    /// The symbol of the sector.
    pub sector_symbol: String,
    /// The type of waypoint.
    #[serde(rename = "type")]
    pub system_type: SystemType,
    /// Relative position of the system in the sector in the x axis.
    pub x: i64,
    /// Relative position of the system in the sector in the y axis.
    pub y: i64,
    /// Waypoints in this system.
    pub distance: i64,
}