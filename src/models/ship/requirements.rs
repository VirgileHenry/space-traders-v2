use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
    power: i64,
    crew: i64,
    slots: i64,
}


