use serde::{Serialize, Deserialize};

/// A surveyed deposit of a mineral or resource available for extraction.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SurveyDeposit {
    /// The symbol of the deposit.
    /// TODO: is it trade symbol?
    pub symbol: String,
}