use serde::Deserialize;

use crate::models::universe::waypoint::RouteWaypoint;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NavStatus {
    InTransit,
    InOrbit,
    Docked,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Nav {
    system_symbol: String,
    waypoint_symbol: String,
    route: NavRoute,
    status: NavStatus,
    flight_mode: FlightMode,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NavRoute {
    destination: RouteWaypoint,
    #[warn(deprecated)]
    departure: RouteWaypoint,
    origin: RouteWaypoint,
    departure_time: String,
    arrival: String,
}