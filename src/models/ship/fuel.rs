use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fuel {
    current: u64,
    capacity: u64,
    consumed: Option<ConsumedFuel>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConsumedFuel {
    amount: u64,
    timestamp: String,
}