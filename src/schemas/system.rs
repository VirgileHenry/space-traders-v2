pub mod system_faction;
pub mod system_type;
pub mod system_waypoint;

use serde::Deserialize;
use self::{
    system_type::SystemType,
    system_waypoint::SystemWaypoint,
    system_faction::SystemFaction
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct System {
    /// The symbol of the system.
    pub symbol: String,
    /// The symbol of the sector.
    pub sector_symbol: String,
    /// The type of waypoint.
    pub system_type: SystemType,
    /// Relative position of the system in the sector in the x axis.
    pub x: i64,
    /// Relative position of the system in the sector in the y axis.
    pub y: i64,
    /// Waypoints in this system.
    pub waypoints: Vec<SystemWaypoint>,
    /// Factions that control this system.
    pub factions: Vec<SystemFaction>,
}