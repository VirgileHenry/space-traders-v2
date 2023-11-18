use serde::Deserialize;

use crate::schemas::ship::{
    ship_registration::Registration,
    ship_nav::ShipNav,
    ship_frame::ShipFrame,
    ship_reactor::ShipReactor,
    ship_engine::ShipEngine,
    ship_mount::ShipMount
};

/// The ship that was scanned. Details include information about the ship that could be detected by the scanner.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    /// The globally unique identifier of the ship in the following format: [AGENT_SYMBOL]-[HEX_ID]
    pub symbol: String,
    /// The public registration information of the ship.
    pub registration: Registration,
    /// The navigation information of the ship.
    pub nav: ShipNav,
    /// The frame of the ship. The frame determines the number of modules and mounting points of the ship, as well as base fuel capacity. As the condition of the frame takes more wear, the ship will become more sluggish and less maneuverable.
    pub frame: Option<ShipFrame>,
    /// The reactor of the ship. The reactor is responsible for powering the ship's systems and weapons.
    pub reactor: Option<ShipReactor>,
    /// The engine determines how quickly a ship travels between waypoints.
    pub engine: ShipEngine,
    /// Mounts installed in this ship.
    pub mounts: Option<Vec<ShipMount>>,
}