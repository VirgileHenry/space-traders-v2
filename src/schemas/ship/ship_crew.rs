use serde::Deserialize;

/// The rotation of crew shifts. A stricter shift improves the ship's performance. A more relaxed shift improves the crew's morale.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipCrewRotation {
    /// Improves the ship's performance.
    Strict,
    /// Improves the crew's morale.
    Relaxed,
}

/// The ship's crew service and maintain the ship's systems and equipment.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipCrew {
    /// The current number of crew members on the ship.
    pub current: i64,
    /// The minimum number of crew members required to maintain the ship.
    pub required: i64,
    /// The maximum number of crew members the ship can support.
    pub capacity: i64,
    /// The rotation of crew shifts. A stricter shift improves the ship's performance. A more relaxed shift improves the crew's morale.
    pub rotation: ShipCrewRotation,
    /// A rough measure of the crew's morale. A higher morale means the crew is happier and more productive. A lower morale means the ship is more prone to accidents.
    pub morale: u64,
    /// The amount of credits per crew member paid per hour. Wages are paid when a ship docks at a civilized waypoint.
    pub wages: u64,
}