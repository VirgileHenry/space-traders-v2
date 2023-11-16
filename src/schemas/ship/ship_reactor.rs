use std::num::NonZeroU64;
use serde::Deserialize;
use super::ship_requirements::ShipRequirements;

/// Symbol of the reactor.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipReactorType {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}

/// The reactor of the ship. The reactor is responsible for powering the ship's systems and weapons.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipReactor {
    /// Symbol of the reactor.
    pub symbol: ShipReactorType,
    /// Name of the reactor.
    pub name: String,
    /// Description of the reactor.
    pub description: String,
    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    pub condition: Option<u64>,
    /// The amount of power provided by this reactor. The more power a reactor provides to the ship, the lower the cooldown it gets when using a module or mount that taxes the ship's power.
    pub power_output: NonZeroU64,
    /// The requirements for installation on a ship
    pub requirements: ShipRequirements,
}


