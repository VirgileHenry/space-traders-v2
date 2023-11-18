use serde::{Serialize, Deserialize};

/// The ship's set speed when traveling between waypoints or systems.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipNavFlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn,
}
