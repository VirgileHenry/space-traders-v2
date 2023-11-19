use serde::{Serialize, Deserialize};

use super::ship_requirements::ShipRequirements;

/// Symbo of this mount.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipMountType {
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

/// Mounts that have this value denote what goods can be produced from using the mount.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipMountDeposits {
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

/// A mount is installed on the exterier of a ship.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipMount {
    /// Symbo of this mount.
    pub symbol: ShipMountType,
    /// Name of this mount.
    pub name: String,
    /// Description of this mount.
    pub description: Option<String>,
    /// Mounts that have this value, such as mining lasers, denote how powerful this mount's capabilities are.
    pub strength: Option<u64>,
    /// Mounts that have this value denote what goods can be produced from using the mount.
    pub deposits: Option<Vec<ShipMountDeposits>>,
    /// The requirements for installation on a ship
    pub requirements: ShipRequirements,
}

