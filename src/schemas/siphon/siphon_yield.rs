use serde::Deserialize;
use crate::schemas::trade_symbol::TradeSymbol;

/// A yield from the siphon operation.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SiphonYield {
    /// Symbol of the good that was siphoned.
    pub symbol: TradeSymbol,
    /// The number of units siphoned that were placed into the ship's cargo hold.
    pub units: i64,
}