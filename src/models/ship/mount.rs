use serde::Deserialize;

use super::requirements::Requirements;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MountSymbol {
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

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MountDeposits {
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
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    symbol: MountSymbol,
    name: String,
    description: Option<String>,
    strength: Option<String>,
    deposits: Option<Vec<MountDeposits>>,
    requirements: Requirements,
}

