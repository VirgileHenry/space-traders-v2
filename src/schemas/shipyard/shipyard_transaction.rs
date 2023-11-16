use serde::Deserialize;
use crate::utils::date_time_string::DateTimeString;

/// Results of a transaction with a shipyard.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardTransaction {
    /// The symbol of the waypoint where the transaction took place.
    pub waypoint_symbol: String,
    /// The symbol of the ship that was the subject of the transaction.
    pub ship_symbol: String,
    /// The price of the transaction.
    pub price: u64,
    /// The symbol of the agent that made the transaction.
    pub agent_symbol: String,
    /// The timestamp of the transaction.
    pub timestamp: DateTimeString,
}