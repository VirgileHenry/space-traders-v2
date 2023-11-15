use serde::Deserialize;

use crate::models::ship::requirements::Requirements;


#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FrameSymbol {
    FrameProbe,
    FrameDrone,
    FrameInterceptor,
    FrameRacer,
    FrameFighter,
    FrameFrigate,
    FrameShuttle,
    FrameExplorer,
    FrameMiner,
    FrameLightFreighter,
    FrameHeavyFreighter,
    FrameTransport,
    FrameDestroyer,
    FrameCruiser,
    FrameCarrier,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    symbol: FrameSymbol,
    name: String,
    description: String,
    condition: Option<u64>,
    module_slots: u64,
    mouting_points: u64,
    fuel_capacity: u64,
    requirements: Requirements,
}


