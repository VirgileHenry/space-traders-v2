pub mod siphon_yield;

use serde::Deserialize;
use self::siphon_yield::SiphonYield;

/// Siphon details.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Siphon {
    /// Symbol of the ship that executed the siphon.
    pub ship_symbol: String,
    /// Yields from the siphon operation.
    #[serde(rename = "yield")]
    pub siphon_yield: SiphonYield,
}