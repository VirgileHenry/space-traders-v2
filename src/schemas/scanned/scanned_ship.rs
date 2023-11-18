use serde::Deserialize;

use crate::schemas::{
    waypoint::{
        waypoint_type::WaypointType,
        waypoint_orbital::WaypointOrbital,
        waypoint_trait::WaypointTrait
    },
    faction::faction_symbol::FactionSymbol,
    chart::Chart
};

/// A waypoint that was scanned by a ship.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    /// Symbol fo the waypoint.
    pub symbol: String,
    /// The type of waypoint.
    #[serde(rename = "type")]
    pub waypoint_type: WaypointType,
    /// The symbol of the system this waypoint belongs to.
    pub system_symbol: String,
    /// Relative position of the waypoint on the system's x axis. This is not an absolute position in the universe.
    pub x: i64,
    /// Relative position of the waypoint on the system's y axis. This is not an absolute position in the universe.
    pub y: i64,
    /// Waypoints that orbit this waypoint.
    pub orbitals: Vec<WaypointOrbital>,
    /// The symbol of the parent waypoint, if this waypoint is in orbit around another waypoint. Otherwise this value is undefined.
    pub faction: FactionSymbol,
    /// The traits of the waypoint.
    pub traits: Vec<WaypointTrait>,
    /// The chart of a system or waypoint, which makes the location visible to other agents.
    pub chart: Option<Chart>,
}