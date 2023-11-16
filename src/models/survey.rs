use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SurveySize {
    Small,
    Moderate,
    Large,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Survey {
    signature: String,
    symbol: String,
    deposits: Vec<Deposit>,
    expiration: String,
    size: SurveySize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    symbol: String,
}