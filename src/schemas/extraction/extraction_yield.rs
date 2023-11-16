use serde::Deserialize;

use crate::schemas::trade_symbol::TradeSymbol;


/// A yield from the extraction operation.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExtractionYield {
    /// Symbol of the good that was extracted.
    pub symbol: TradeSymbol,
    /// The number of units extracted that were placed into the ship's cargo hold.
    pub units: i64,
}