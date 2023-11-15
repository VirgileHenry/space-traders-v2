use serde::Deserialize;

use crate::models::faction::FactionSymbol;

use super::chart::Chart;


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Asteroid,
    EngineeredAsteroid,
    AsteroidBase,
    Nebula,
    DebrisField,
    GravityWell,
    ArtificialGravityWell,
    FuelStation,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointTraitType {
    Uncharted,
    UnderConstruction,
    Marketplace,
    Shipyard,
    Outpost,
    ScatteredSettlements,
    SprawlingCities,
    MegaStructures,
    Overcroweded,
    HighTech,
    Corrupt,
    Bureaucratic,
    TradingHub,
    Industrial,
    BlackMarket,
    ResearchFacility,
    MilitaryBase,
    SurveillanceOutpost,
    ExplorationOutpost,
    MineralDeposits,
    CommonMetalDeposits,
    MethanePools,
    IceCrystals,
    ExplosiveGases,
    StrongMagnetosphere,
    VibrantAuroras,
    SaltFlats,
    Canyons,
    PerpetualDaylight,
    PerpetualOvercast,
    DrySeabeds,
    MagmaSeas,
    Supervolcanoes,
    AshCloud,
    VastRuins,
    MutatedFlora,
    Terraformed,
    ExtremeTemperatures,
    ExtremePressure,
    DiverseLife,
    ScarceLife,
    Fossils,
    WeakGravity,
    StrongGravity,
    CrushingGravity,
    ToxicAtmosphere,
    CorrosiveAtmosphere,
    BreathableAtmosphere,
    ThinAtmosphere,
    Jovian,
    Rocky,
    Volcanic,
    Frozen,
    Swamp,
    Barren,
    Temperate,
    Jungle,
    Ocean,
    Radioactive,
    MicroGravityAnomalies,
    DebrisCluster,
    DeepCraters,
    ShallowCraters,
    UnstableComposition,
    HollowedInterior,
    Stripped,
}



#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointModifierType {
    Stripped,
    Unstable,
    RadiationLeak,
    CriticalLimit,
    CivilUnrest,
}

/// Shorter representation of a waypoint used for routes.
/// Does not contain all waypoint info.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RouteWaypoint {
    symbol: String,
    #[serde(rename = "type")]
    waypoint_type: WaypointType,
    system_symbol: String,
    x: i64,
    y: i64,
}

/// Full waypoint info.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    symbol: String,
    #[serde(rename = "type")]
    waypoint_type: WaypointType,
    system_symbol: String,
    x: i64,
    y: i64,
    orbitals: Vec<WaypointSymbol>,
    orbits: Option<WaypointSymbol>,
    faction: FactionSymbol,
    traits: Vec<WaypointTrait>,
    modifiers: Option<Vec<WaypointModifier>>,
    chart: Option<Chart>,
    is_under_construction: bool,
}


/// Shorter representation of a waypoint used for routes.
/// Does not contain all waypoint info.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WaypointSymbol {
    symbol: String,
}


/// Representation of a waypoint trait.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WaypointTrait {
    symbol: WaypointTraitType,
    name: String,
    description: String,
}


/// Representation of a waypoint modifier
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WaypointModifier {
    symbol: WaypointModifierType,
    name: String,
    description: String,
}


