use serde::Deserialize;

use crate::models::resource::Resource;

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

pub struct NotAModuleError;
impl TryFrom<Resource> for ModuleSymbol {
    type Error = NotAModuleError;
    fn try_from(value: Resource) -> Result<Self, Self::Error> {
        match value {
            Resource::ModuleMineralProcessorI => Ok(ModuleSymbol::ModuleMineralProcessorI),
            Resource::ModuleGasProcessorI => Ok(ModuleSymbol::ModuleGasProcessorI),
            Resource::ModuleCargoHoldI => Ok(ModuleSymbol::ModuleCargoHoldI),
            Resource::ModuleCargoHoldII => Ok(ModuleSymbol::ModuleCargoHoldII),
            Resource::ModuleCargoHoldIII => Ok(ModuleSymbol::ModuleCargoHoldIII),
            Resource::ModuleCrewQuartersI => Ok(ModuleSymbol::ModuleCrewQuartersI),
            Resource::ModuleEnvoyQuartersI => Ok(ModuleSymbol::ModuleEnvoyQuartersI),
            Resource::ModulePassengerCabinI => Ok(ModuleSymbol::ModulePassengerCabinI),
            Resource::ModuleMicroRefineryI => Ok(ModuleSymbol::ModuleMicroRefineryI),
            Resource::ModuleScienceLabI => Ok(ModuleSymbol::ModuleScienceLabI),
            Resource::ModuleJumpDriveI => Ok(ModuleSymbol::ModuleJumpDriveI),
            Resource::ModuleJumpDriveII => Ok(ModuleSymbol::ModuleJumpDriveII),
            Resource::ModuleJumpDriveIII => Ok(ModuleSymbol::ModuleJumpDriveIII),
            Resource::ModuleWarpDriveI => Ok(ModuleSymbol::ModuleWarpDriveI),
            Resource::ModuleWarpDriveII => Ok(ModuleSymbol::ModuleWarpDriveII),
            Resource::ModuleWarpDriveIII => Ok(ModuleSymbol::ModuleWarpDriveIII),
            Resource::ModuleShieldGeneratorI => Ok(ModuleSymbol::ModuleShieldGeneratorI),
            Resource::ModuleShieldGeneratorII => Ok(ModuleSymbol::ModuleShieldGeneratorII),
            _ => Err(NotAModuleError),
        }
    }
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

