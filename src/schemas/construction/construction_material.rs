use serde::Deserialize;
use crate::schemas::trade_symbol::TradeSymbol;

/// The details of the required construction materials for a given waypoint under construction.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ConstructionMaterial {
    /// The good's symbol.
    pub trade_symbol: TradeSymbol,
    /// The number of units required.
    pub required: i64,
    /// The number of units fulfilled toward the required amount.
    pub fulfilled: i64,
}