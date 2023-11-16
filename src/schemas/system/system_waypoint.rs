use serde::Deserialize;
use crate::schemas::waypoint::{
    waypoint_type::WaypointType,
    waypoint_orbital::WaypointOrbital
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemWaypoint {
    /// The symbol of the waypoint.
    pub symbol: String,
    /// The type of waypoint.
    pub waypoint_type: WaypointType,
    /// Relative position of the waypoint on the system's x axis. This is not an absolute position in the universe.
    pub x: i64,
    /// Relative position of the waypoint on the system's y axis. This is not an absolute position in the universe.
    pub y: i64,
    /// Waypoints that orbit this waypoint.
    pub orbitals: Vec<WaypointOrbital>,
    /// The symbol of the parent waypoint, if this waypoint is in orbit around another waypoint. Otherwise this value is undefined.
    pub orbits: Option<String>,
}