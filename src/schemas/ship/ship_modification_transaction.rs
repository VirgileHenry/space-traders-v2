use serde::Deserialize;


/// Result of a transaction for a ship modification, such as installing a mount or a module.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipModificationTransaction {
    /// The symbol of the waypoint where the transaction took place.
    pub waypoint_symbol: String,
    /// The symbol of the ship that made the transaction.
    pub ship_symbol: String,
    /// The symbol of the trade good.
    pub trade_symbol: String,
    /// The total price of the transaction.
    pub total_price: u64,
    /// The timestamp of the transaction.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}