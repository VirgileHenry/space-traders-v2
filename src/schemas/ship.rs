pub mod ship_cargo;
pub mod ship_condition;
pub mod ship_crew;
pub mod ship_engine;
pub mod ship_frame;
pub mod ship_fuel;
pub mod ship_modification_transaction;
pub mod ship_module;
pub mod ship_mount;
pub mod ship_nav;
pub mod ship_reactor;
pub mod ship_registration;
pub mod ship_requirements;
pub mod ship_role;
pub mod ship_type;

use serde::Deserialize;
use super::cooldown::Cooldown;
use self::{
    ship_registration::Registration,
    ship_nav::ShipNav,
    ship_crew::ShipCrew,
    ship_frame::ShipFrame,
    ship_reactor::ShipReactor,
    ship_engine::ShipEngine,
    ship_module::ShipModule,
    ship_mount::ShipMount,
    ship_cargo::ShipCargo,
    ship_fuel::ShipFuel
};

/// Ship details.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    /// The globally unique identifier of the ship in the following format: [AGENT_SYMBOL]-[HEX_ID]
    pub symbol: String,
    /// The public registration information of the ship.
    pub registration: Registration,
    /// The navigation information of the ship.
    pub nav: ShipNav,
    /// The ship's crew service and maintain the ship's systems and equipment.
    pub crew: ShipCrew,
    /// The frame of the ship. The frame determines the number of modules and mounting points of the ship, as well as base fuel capacity. As the condition of the frame takes more wear, the ship will become more sluggish and less maneuverable.
    pub frame: ShipFrame,
    /// The reactor of the ship. The reactor is responsible for powering the ship's systems and weapons.
    pub reactor: ShipReactor,
    /// The engine determines how quickly a ship travels between waypoints.
    pub engine: ShipEngine,
    /// A cooldown is a period of time in which a ship cannot perform certain actions.
    pub cooldown: Cooldown,
    /// Modules installed in this ship.
    pub modules: Vec<ShipModule>,
    /// Mounts installed in this ship.
    pub mounts: Vec<ShipMount>,
    /// Ship cargo details.
    pub cargo: ShipCargo,
    /// Details of the ship's fuel tanks including how much fuel was consumed during the last transit or action.
    pub fuel: ShipFuel,
}