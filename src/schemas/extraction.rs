pub mod extraction_yield;

use serde::Deserialize;
use self::extraction_yield::ExtractionYield;

/// Extraction details.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Extraction {
    /// Symbol of the ship that executed the extraction.
    ship_symbol: String,
    /// Yields from the extract operation.
    #[serde(rename = "yield")]
    extraction_yield: ExtractionYield,
}