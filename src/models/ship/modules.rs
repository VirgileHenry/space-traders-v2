use serde::Deserialize;

use super::requirements::Requirements;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModuleSymbol {
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


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    symbol: ModuleSymbol,
    capacity: Option<u64>,
    range: Option<u64>,
    name: String,
    description: String,
    requirements: Requirements,
}

