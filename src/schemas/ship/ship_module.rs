use serde::Deserialize;
use super::ship_requirements::ShipRequirements;

/// The symbol of the module.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipModuleType {
    ModuleMineralProcessorI,
    ModuleGasProcessorI,
    ModuleCargoHoldI,
    ModuleCargoHoldII,
    ModuleCargoHoldIII,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    ModuleJumpDriveII,
    ModuleJumpDriveIII,
    ModuleWarpDriveI,
    ModuleWarpDriveII,
    ModuleWarpDriveIII,
    ModuleShieldGeneratorI,
    ModuleShieldGeneratorII,
}

/// A module can be installed in a ship and provides a set of capabilities such as storage space or quarters for crew. Module installations are permanent.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipModule {
    /// The symbol of the module.
    pub symbol: ShipModuleType,
    /// Modules that provide capacity, such as cargo hold or crew quarters will show this value to denote how much of a bonus the module grants.
    pub capacity: Option<u64>,
    /// Modules that have a range will such as a sensor array show this value to denote how far can the module reach with its capabilities.
    pub range: Option<u64>,
    /// Name of this module.
    pub name: String,
    /// Description of this module.
    pub description: String,
    /// The requirements for installation on a ship
    pub requirements: ShipRequirements,
}

