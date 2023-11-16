use serde::Deserialize;

use crate::models::resource::Resource;

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

pub struct NotAMountError;
impl TryFrom<Resource> for MountSymbol {
    type Error = NotAMountError;
    fn try_from(value: Resource) -> Result<Self, Self::Error> {
        match value {
            Resource::MountGasSiphonI => Ok(MountSymbol::MountGasSiphonI),
            Resource::MountGasSiphonII => Ok(MountSymbol::MountGasSiphonII),
            Resource::MountGasSiphonIII => Ok(MountSymbol::MountGasSiphonIII),
            Resource::MountSurveyorI => Ok(MountSymbol::MountSurveyorI),
            Resource::MountSurveyorII => Ok(MountSymbol::MountSurveyorII),
            Resource::MountSurveyorIII => Ok(MountSymbol::MountSurveyorIII),
            Resource::MountSenserArrayI => Ok(MountSymbol::MountSenserArrayI),
            Resource::MountSenserArrayII => Ok(MountSymbol::MountSenserArrayII),
            Resource::MountSenserArrayIII => Ok(MountSymbol::MountSenserArrayIII),
            Resource::MountMiningLaserI => Ok(MountSymbol::MountMiningLaserI),
            Resource::MountMiningLaserII => Ok(MountSymbol::MountMiningLaserII),
            Resource::MountMiningLaserIII => Ok(MountSymbol::MountMiningLaserIII),
            Resource::MountLaserCanonI => Ok(MountSymbol::MountLaserCanonI),
            Resource::MountMissileLauncherI => Ok(MountSymbol::MountMissileLauncherI),
            Resource::MountTurretI => Ok(MountSymbol::MountTurretI),
            _ => Err(NotAMountError),
        }
    }
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

