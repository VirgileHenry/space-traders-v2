use serde::Deserialize;

use super::ship::{engine::EngineSymbol, modules::ModuleSymbol, mount::MountSymbol, reactor::ReactorSymbol, refining::RefinedGoodType};


#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Resource {
    // mounts deposits
    QuartzSand,
    SiliconCrystals,
    PreciousStones,
    IceWater,
    AmmoniaIce,
    IronOre,
    CopperOre,
    SilverOre,
    AluminiumOre,
    GoldOre,
    PlatiniumOre,
    Diamonds,
    UraniteOre,
    MeritiumOre,
    // refined materials
    Iron,
    Copper,
    Silver,
    Gold,
    Aluminium,
    Platinium,
    Uranite,
    Meritium,
    Fuel,
    // reactors
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
    // engines 
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveII,
    EngineHyperDriveI,
    // modules
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
    // mounts
    MountGasSiphonI,
    MountGasSiphonII,
    MountGasSiphonIII,
    MountSurveyorI,
    MountSurveyorII,
    MountSurveyorIII,
    MountSenserArrayI,
    MountSenserArrayII,
    MountSenserArrayIII,
    MountMiningLaserI,
    MountMiningLaserII,
    MountMiningLaserIII,
    MountLaserCanonI,
    MountMissileLauncherI,
    MountTurretI,
}

impl From<EngineSymbol> for Resource {
    fn from(value: EngineSymbol) -> Self {
        match value {
            EngineSymbol::EngineIonDriveI => Resource::EngineIonDriveI,
            EngineSymbol::EngineIonDriveII => Resource::EngineIonDriveII,
            EngineSymbol::EngineHyperDriveI => Resource::EngineHyperDriveI,
            EngineSymbol::EngineImpulseDriveI => Resource::EngineImpulseDriveI,
        }
    }
}

impl From<ModuleSymbol> for Resource {
    fn from(value: ModuleSymbol) -> Self {
        match value {
            ModuleSymbol::ModuleMineralProcessorI => Resource::ModuleMineralProcessorI,
            ModuleSymbol::ModuleGasProcessorI => Resource::ModuleGasProcessorI,
            ModuleSymbol::ModuleCargoHoldI => Resource::ModuleCargoHoldI,
            ModuleSymbol::ModuleCargoHoldII => Resource::ModuleCargoHoldII,
            ModuleSymbol::ModuleCargoHoldIII => Resource::ModuleCargoHoldIII,
            ModuleSymbol::ModuleCrewQuartersI => Resource::ModuleCrewQuartersI,
            ModuleSymbol::ModuleEnvoyQuartersI => Resource::ModuleEnvoyQuartersI,
            ModuleSymbol::ModulePassengerCabinI => Resource::ModulePassengerCabinI,
            ModuleSymbol::ModuleMicroRefineryI => Resource::ModuleMicroRefineryI,
            ModuleSymbol::ModuleScienceLabI => Resource::ModuleScienceLabI,
            ModuleSymbol::ModuleJumpDriveI => Resource::ModuleJumpDriveI,
            ModuleSymbol::ModuleJumpDriveII => Resource::ModuleJumpDriveII,
            ModuleSymbol::ModuleJumpDriveIII => Resource::ModuleJumpDriveIII,
            ModuleSymbol::ModuleWarpDriveI => Resource::ModuleWarpDriveI,
            ModuleSymbol::ModuleWarpDriveII => Resource::ModuleWarpDriveII,
            ModuleSymbol::ModuleWarpDriveIII => Resource::ModuleWarpDriveIII,
            ModuleSymbol::ModuleShieldGeneratorI => Resource::ModuleShieldGeneratorI,
            ModuleSymbol::ModuleShieldGeneratorII => Resource::ModuleShieldGeneratorII,
        }
    }
}

impl From<MountSymbol> for Resource {
    fn from(value: MountSymbol) -> Self {
        match value {
            MountSymbol::MountGasSiphonI => Resource::MountGasSiphonI,
            MountSymbol::MountGasSiphonII => Resource::MountGasSiphonII,
            MountSymbol::MountGasSiphonIII => Resource::MountGasSiphonIII,
            MountSymbol::MountSurveyorI => Resource::MountSurveyorI,
            MountSymbol::MountSurveyorII => Resource::MountSurveyorII,
            MountSymbol::MountSurveyorIII => Resource::MountSurveyorIII,
            MountSymbol::MountSenserArrayI => Resource::MountSenserArrayI,
            MountSymbol::MountSenserArrayII => Resource::MountSenserArrayII,
            MountSymbol::MountSenserArrayIII => Resource::MountSenserArrayIII,
            MountSymbol::MountMiningLaserI => Resource::MountMiningLaserI,
            MountSymbol::MountMiningLaserII => Resource::MountMiningLaserII,
            MountSymbol::MountMiningLaserIII => Resource::MountMiningLaserIII,
            MountSymbol::MountLaserCanonI => Resource::MountLaserCanonI,
            MountSymbol::MountMissileLauncherI => Resource::MountMissileLauncherI,
            MountSymbol::MountTurretI => Resource::MountTurretI,
        }
    }
}

impl From<ReactorSymbol> for Resource {
    fn from(value: ReactorSymbol) -> Self {
        match value {
            ReactorSymbol::ReactorSolarI => Resource::ReactorSolarI,
            ReactorSymbol::ReactorFusionI => Resource::ReactorFusionI,
            ReactorSymbol::ReactorFissionI => Resource::ReactorFissionI,
            ReactorSymbol::ReactorChemicalI => Resource::ReactorChemicalI,
            ReactorSymbol::ReactorAntimatterI => Resource::ReactorAntimatterI,
        }
    }
}

impl From<RefinedGoodType> for Resource {
    fn from(value: RefinedGoodType) -> Self {
        match value {
            RefinedGoodType::Iron => Resource::Iron,
            RefinedGoodType::Copper => Resource::Copper,
            RefinedGoodType::Silver => Resource::Silver,
            RefinedGoodType::Gold => Resource::Gold,
            RefinedGoodType::Aluminium => Resource::Aluminium,
            RefinedGoodType::Uranite => Resource::Uranite,
            RefinedGoodType::Platinium => Resource::Platinium,
            RefinedGoodType::Fuel => Resource::Fuel,
            RefinedGoodType::Meritium => Resource::Meritium,
        }
    }
}