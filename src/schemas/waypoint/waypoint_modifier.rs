use serde::Deserialize;

/// The unique identifier of the modifier.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointModifierType {
    Stripped,
    Unstable,
    RadiationLeak,
    CriticalLimit,
    CivilUnrest,
}


/// Representation of a waypoint modifier
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WaypointModifier {
    /// The unique identifier of the modifier.
    pub symbol: WaypointModifierType,
    /// The name of the trait.
    pub name: String,
    /// A description of the trait.
    pub description: String,
}


