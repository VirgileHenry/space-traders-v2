use serde::Deserialize;



#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    waypoint_symbol: String,
    submitted_by: String,
    submitted_on: String,
}




