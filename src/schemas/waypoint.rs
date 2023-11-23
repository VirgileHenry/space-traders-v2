pub mod waypoint_faction;
pub mod waypoint_modifier;
pub mod waypoint_orbital;
pub mod waypoint_trait;
pub mod waypoint_type;

use serde::Deserialize;
use self::{
    waypoint_type::WaypointType,
    waypoint_orbital::WaypointOrbital,
    waypoint_trait::WaypointTrait,
    waypoint_modifier::WaypointModifier, waypoint_faction::WaypointFaction
};
use super::chart::Chart;

/// A waypoint is a location that ships can travel to such as a Planet, Moon or Space Station.
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
    pub orbits: Option<String>,
    /// The faction that controls the waypoint.
    pub faction: WaypointFaction,
    /// The traits of the waypoint.
    pub traits: Vec<WaypointTrait>,
    /// The modifiers of the waypoint.
    pub modifiers: Option<Vec<WaypointModifier>>,
    /// The chart of a system or waypoint, which makes the location visible to other agents.
    pub chart: Option<Chart>,
    /// True if the waypoint is under construction.
    pub is_under_construction: bool,
}

