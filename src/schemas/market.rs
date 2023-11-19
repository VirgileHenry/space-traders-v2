pub mod market_trade_good;
pub mod market_transaction;

use serde::Deserialize;
use self::{
    market_transaction::MarketTransaction,
    market_trade_good::MarketTradeGood
};
use super::trade_good::TradeGood;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    /// The symbol of the market. The symbol is the same as the waypoint where the market is located.
    pub symbol: String,
    /// The list of goods that are exported from this market.
    pub exports: Vec<TradeGood>,
    /// The list of goods that are sought as imports in this market.
    pub imports: Vec<TradeGood>,
    /// The list of goods that are bought and sold between agents at this market.
    pub exchange: Vec<TradeGood>,
    /// The list of recent transactions at this market. Visible only when a ship is present at the market.
    pub transactions: Option<Vec<MarketTransaction>>,
    /// The list of goods that are traded at this market. Visible only when a ship is present at the market.
    pub trade_goods: Option<Vec<MarketTradeGood>>,
}