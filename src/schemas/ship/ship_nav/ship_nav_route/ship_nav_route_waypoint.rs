use serde::Deserialize;
use crate::schemas::waypoint::waypoint_type::WaypointType;

/// The destination or departure of a ships nav route.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRouteWaypoint {
    /// The symbol of the waypoint.
    pub symbol: String,
    /// The type of waypoint.
    #[serde(rename = "type")]
    pub waypoint_type: WaypointType,
    /// The symbol of the system the waypoint is in.
    pub system_symbol: String,
    /// Position in the universe in the x axis.
    pub x: i64,
    /// Position in the universe in the y axis.
    pub y: i64,
}
