use serde::Deserialize;
use super::ship_requirements::ShipRequirements;

/// Symbol of the frame.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipFrameType {
    FrameProbe,
    FrameDrone,
    FrameInterceptor,
    FrameRacer,
    FrameFighter,
    FrameFrigate,
    FrameShuttle,
    FrameExplorer,
    FrameMiner,
    FrameLightFreighter,
    FrameHeavyFreighter,
    FrameTransport,
    FrameDestroyer,
    FrameCruiser,
    FrameCarrier,
}

/// The frame of the ship. The frame determines the number of modules and mounting points of the ship, as well as base fuel capacity. As the condition of the frame takes more wear, the ship will become more sluggish and less maneuverable.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    /// Symbol of the frame.
    pub symbol: ShipFrameType,
    /// Name of the frame.
    pub name: String,
    /// Description of the frame.
    pub description: String,
    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    pub condition: Option<u64>,
    /// The amount of slots that can be dedicated to modules installed in the ship. Each installed module take up a number of slots, and once there are no more slots, no new modules can be installed.
    pub module_slots: u64,
    /// The amount of slots that can be dedicated to mounts installed in the ship. Each installed mount takes up a number of points, and once there are no more points remaining, no new mounts can be installed.
    pub mouting_points: u64,
    /// The maximum amount of fuel that can be stored in this ship. When refueling, the ship will be refueled to this amount.
    pub fuel_capacity: u64,
    /// The requirements for installation on a ship
    pub requirements: ShipRequirements,
}


