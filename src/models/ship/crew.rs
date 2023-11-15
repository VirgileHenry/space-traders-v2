use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CrewRotation {
    Strict,
    Relaxed,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Crew {
    #[serde(rename = "current")]
    current_number: i64,
    required: i64,
    capacity: i64,
    rotation: CrewRotation,
    morale: u64,
    wages: u64,
}