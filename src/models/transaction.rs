use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    waypoint_symbol: String,
    ship_symbol: String,
    price: u64,
    agent_symbol: String,
    time_stamp: String,
}