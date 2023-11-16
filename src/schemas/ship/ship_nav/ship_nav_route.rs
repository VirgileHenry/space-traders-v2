pub mod ship_nav_route_waypoint;

use serde::Deserialize;
use crate::utils::date_time_string::DateTimeString;
use self::ship_nav_route_waypoint::ShipNavRouteWaypoint;

/// The routing information for the ship's most recent transit or current location.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRoute {
    /// The destination or departure of a ships nav route.
    pub destination: ShipNavRouteWaypoint,
    /// Deprecated. Use origin instead.
    #[warn(deprecated)]
    pub departure: ShipNavRouteWaypoint,
    /// The destination or departure of a ships nav route.
    pub origin: ShipNavRouteWaypoint,
    /// The date time of the ship's departure.
    pub departure_time: DateTimeString,
    /// The date time of the ship's arrival. If the ship is in-transit, this is the expected time of arrival.
    pub arrival: String,
}