use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    ship_symbol: String,
    total_seconds: u64,
    remaining_seconds: u64,
    expiration: Option<String>,
}


