use serde::Deserialize;
use crate::utils::date_time_string::DateTimeString;

/// The activity level of a trade good. If the good is an import, this represents how strong consumption is for the good. If the good is an export, this represents how strong the production is for the good.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketTransactionType {
    Purchase,
    Sell,
}

/// Result of a transaction with a market.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketTransaction {
    /// The symbol of the waypoint where the transaction took place.
    pub waypoint_symbol: String,
    /// The symbol of the ship that made the transaction.
    pub ship_symbol: String,
    /// The symbol of the trade good.
    pub trade_symbol: String,
    /// The type of transaction.
    #[serde(rename = "type")]
    pub transaction_type: MarketTransactionType,
    /// The number of units of the transaction.
    pub units: u64,
    /// The price per unit of the transaction.
    pub price_per_unit: u64,
    /// The total price of the transaction.
    pub total_price: u64,
    /// The timestamp of the transaction.
    pub timestamp: DateTimeString,
}