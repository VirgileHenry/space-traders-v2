use serde::Deserialize;


/// An orbital is another waypoint that orbits a parent waypoint.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WaypointOrbital {
    /// The symbol of the orbiting waypoint.
    pub symbol: String,
}