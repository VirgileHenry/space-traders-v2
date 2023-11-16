use std::num::NonZeroU64;
use serde::Deserialize;
use super::ship_requirements::ShipRequirements;

/// The symbol of the engine.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipEngineType {
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveII,
    EngineHyperDriveI,
}

/// The engine determines how quickly a ship travels between waypoints.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipEngine {
    /// The symbol of the engine.
    pub symbol: ShipEngineType,
    /// The name of the engine.
    pub name: String,
    /// The description of the engine.
    pub description: String,
    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    pub condition: Option<u64>,
    /// The speed stat of this engine. The higher the speed, the faster a ship can travel from one point to another. Reduces the time of arrival when navigating the ship.
    pub speed: NonZeroU64,
    /// The requirements for installation on a ship
    pub requirements: ShipRequirements,
}
