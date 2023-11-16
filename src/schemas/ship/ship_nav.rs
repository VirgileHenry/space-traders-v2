pub mod ship_nav_flight_mode;
pub mod ship_nav_route;
pub mod ship_nav_status;

use serde::Deserialize;
use self::{
    ship_nav_status::ShipNavStatus,
    ship_nav_flight_mode::ShipNavFlightMode,
    ship_nav_route::ShipNavRoute
};

/// The navigation information of the ship.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Nav {
    /// The system symbol of the ship's current location.
    pub system_symbol: String,
    /// The waypoint symbol of the ship's current location, or if the ship is in-transit, the waypoint symbol of the ship's destination.
    pub waypoint_symbol: String,
    /// The routing information for the ship's most recent transit or current location.
    pub route: ShipNavRoute,
    /// The current status of the ship
    pub status: ShipNavStatus,
    /// The ship's set speed when traveling between waypoints or systems.
    pub flight_mode: ShipNavFlightMode,
}