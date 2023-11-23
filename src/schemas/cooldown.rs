use serde::Deserialize;



/// A cooldown is a period of time in which a ship cannot perform certain actions.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    /// The symbol of the ship that is on cooldown
    pub ship_symbol: String,
    /// The total duration of the cooldown in seconds
    pub total_seconds: u64,
    /// The remaining duration of the cooldown in seconds
    pub remaining_seconds: u64,
    /// The date and time when the cooldown expires in ISO 8601 format
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
}


