use serde::Deserialize;
use super::trade_symbol::TradeSymbol;

/// A good that can be traded for other goods or currency.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeGood {
    /// The good's symbol.
    symbol: TradeSymbol,
    /// The name of the good.
    name: String,
    /// The description of the good.
    description: String,
}