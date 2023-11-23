use serde::Deserialize;

/// The ship's crew service and maintain the ship's systems and equipment.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardShipCrew {
    /// The minimum number of crew members required to maintain the ship.
    pub required: i64,
    /// The maximum number of crew members the ship can support.
    pub capacity: i64,
}